/// A connection to a message bus.
pub struct Connection {
	reader: std::io::BufReader<std::os::unix::net::UnixStream>,
	read_buf: Vec<u8>,
	read_end: usize,
	writer: std::os::unix::net::UnixStream,
	write_buf: Vec<u8>,
	write_endianness: crate::proto::Endianness,
	server_guid: Vec<u8>,
}

/// The path of a message bus.
#[derive(Clone, Copy, Debug)]
pub enum BusPath<'a> {
	/// The session bus. Its path will be determined from the `DBUS_SESSION_BUS_ADDRESS` environment variable.
	Session,

	/// The system bus. Its path will be determined from the `DBUS_SYSTEM_BUS_ADDRESS` environment variable if it exists,
	/// with a fallback to `unix:path=/var/run/dbus/system_bus_socket` if it doesn't.
	System,

	/// A unix domain socket file at the specified filesystem path.
	UnixSocketFile(&'a std::path::Path),
}

/// The string to send for SASL EXTERNAL authentication with the message bus.
///
/// `Uid` is usually the type to use for local message buses.
#[derive(Clone, Copy, Debug)]
pub enum SaslAuthType<'a> {
	/// The user ID of the current thread will be used.
	Uid,

	/// The specified string will be used.
	Other(&'a str),
}

impl Connection {
	/// Opens a connection to the bus at the given path with the given authentication type.
	pub fn new(
		bus_path: BusPath<'_>,
		sasl_auth_type: SaslAuthType<'_>,
	) -> Result<Self, ConnectError> {
		use std::io::{BufRead, Write};

		let stream = match bus_path {
			BusPath::Session => {
				let bus_address = std::env::var_os("DBUS_SESSION_BUS_ADDRESS").ok_or(ConnectError::MissingSessionBusEnvVar)?;
				connect(&bus_address)?
			},

			BusPath::System => {
				let bus_address =
					std::env::var_os("DBUS_SYSTEM_BUS_ADDRESS")
					.unwrap_or_else(|| "unix:path=/var/run/dbus/system_bus_socket".into());
				connect(&bus_address)?
			},

			BusPath::UnixSocketFile(bus_path) => {
				let stream =
					std::os::unix::net::UnixStream::connect(bus_path)
					.map_err(|err| ConnectError::Connect(vec![(bus_path.to_owned(), err)]))?;
				stream
			},
		};

		let sasl_auth_id = match sasl_auth_type {
			SaslAuthType::Uid => &{
				let uid = (unsafe { libc::getuid() }).to_string();
				let mut sasl_auth_id = String::with_capacity(uid.len() * 2);
				for c in uid.chars() {
					use std::fmt::Write;
					write!(sasl_auth_id, "{:02x}", c as u32).expect("cannot fail");
				}
				sasl_auth_id
			},

			SaslAuthType::Other(sasl_auth_id) => sasl_auth_id,
		};

		let reader = stream.try_clone().map_err(ConnectError::Authenticate)?;
		let mut reader = std::io::BufReader::new(reader);
		let mut read_buf = vec![];

		let mut writer = stream;
		let write_buf = vec![];

		#[allow(clippy::write_with_newline)]
		write!(writer, "\0AUTH EXTERNAL {sasl_auth_id}\r\n").map_err(ConnectError::Authenticate)?;
		writer.flush().map_err(ConnectError::Authenticate)?;

		let _ = reader.read_until(b'\n', &mut read_buf).map_err(ConnectError::Authenticate)?;
		if read_buf.iter().rev().nth(1).copied() != Some(b'\r') {
			return Err(ConnectError::Authenticate(std::io::Error::other("malformed response")));
		}

		let server_guid =
			if read_buf.starts_with(b"OK ") {
				&read_buf[b"OK ".len()..][..32]
			}
			else {
				return Err(ConnectError::Authenticate(std::io::Error::other("malformed response")));
			};
		let server_guid = server_guid.to_owned();

		read_buf.clear();
		read_buf.resize(1, 0);

		writer.write_all(b"BEGIN\r\n").map_err(ConnectError::Authenticate)?;
		writer.flush().map_err(ConnectError::Authenticate)?;

		// Default to target endianness
		let write_endianness = if cfg!(target_endian = "big") { crate::proto::Endianness::Big } else { crate::proto::Endianness::Little };

		Ok(Connection {
			reader,
			read_buf,
			read_end: 0,
			writer,
			write_buf,
			write_endianness,
			server_guid,
		})
	}

	/// The GUID of the server.
	pub fn server_guid(&self) -> &[u8] {
		&self.server_guid
	}

	/// Send a message with the given header and body to the message bus.
	///
	/// - Header fields corresponding to the required properties of the message type will be automatically inserted, and *must not* be inserted by the caller.
	///   For example, if `header.type` is `MethodCall { member, path }`, the `MessageHeaderField::Member` and `MessageHeaderField::Path` fields
	///   will be inserted automatically.
	///
	/// - The `MessageHeaderField::Signature` field will be automatically inserted if a body is specified, and must not be inserted by the caller.
	pub fn send(&mut self, header: &mut crate::proto::MessageHeader<'_>, body: Option<&crate::proto::Variant<'_>>) -> Result<(), SendError> {
		use std::io::Write;

		let () = crate::proto::serialize_message(header, body, &mut self.write_buf, self.write_endianness).map_err(SendError::Serialize)?;

		let () = self.writer.write_all(&self.write_buf).map_err(SendError::Io)?;
		self.write_buf.clear();

		let () = self.writer.flush().map_err(SendError::Io)?;

		Ok(())
	}

	/// Receive a message from the message bus.
	pub fn recv(&mut self) -> Result<(crate::proto::MessageHeader<'static>, Option<crate::proto::Variant<'static>>), RecvError> {
		use std::io::Read;

		loop {
			match crate::proto::deserialize_message(&self.read_buf[..self.read_end]) {
				Ok((message_header, message_body, read)) => {
					let message_header = message_header.into_owned();
					let message_body = message_body.map(crate::proto::Variant::into_owned);
					self.read_buf.copy_within(read..self.read_end, 0);
					self.read_end -= read;
					return Ok((message_header, message_body));
				},

				Err(crate::proto::DeserializeError::EndOfInput) => {
					if self.read_end == self.read_buf.len() {
						self.read_buf.resize(self.read_buf.len() * 2, 0);
					}

					let read = self.reader.read(&mut self.read_buf[self.read_end..]).map_err(RecvError::Io)?;
					if read == 0 {
						return Err(RecvError::Io(std::io::ErrorKind::UnexpectedEof.into()));
					}

					self.read_end += read;
				},

				Err(err) => return Err(RecvError::Deserialize(err)),
			}
		}
	}

	/// Set the endianness used for sending messages.
	///
	/// By default, the connection uses the target endianness. Use this method to override that.
	pub fn set_write_endianness(&mut self, endianness: crate::proto::Endianness) {
		self.write_endianness = endianness;
	}
}

/// An error from connecting to a message bus.
#[derive(Debug)]
pub enum ConnectError {
	Authenticate(std::io::Error),

	Connect(Vec<(std::path::PathBuf, std::io::Error)>),

	MissingSessionBusEnvVar,

	UnsupportedTransport(std::ffi::OsString),
}

impl std::fmt::Display for ConnectError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConnectError::Authenticate(_) => f.write_str("could not authenticate with bus"),

			ConnectError::Connect(inner) => {
				f.write_str("could not connect to any bus paths: [")?;
				for (i, (bus_path, err)) in inner.iter().enumerate() {
					if i > 0 {
						f.write_str(", ")?;
					}

					write!(f, "{:?}: {:?}", bus_path.display(), err.to_string())?;
				}
				f.write_str("]")?;
				Ok(())
			},

			ConnectError::MissingSessionBusEnvVar => f.write_str("the DBUS_SESSION_BUS_ADDRESS env var is not set"),

			ConnectError::UnsupportedTransport(value) => write!(f, "the bus path {:?} has an unsupported transport", value.display()),
		}
	}
}

impl std::error::Error for ConnectError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		#[allow(clippy::match_same_arms)]
		match self {
			ConnectError::Authenticate(err) => Some(err),
			ConnectError::Connect(_) => None,
			ConnectError::MissingSessionBusEnvVar => None,
			ConnectError::UnsupportedTransport(_) => None,
		}
	}
}

/// An error from sending a message using a [`Connection::send`].
#[derive(Debug)]
pub enum SendError {
	Io(std::io::Error),
	Serialize(crate::proto::SerializeError),
}

impl std::fmt::Display for SendError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SendError::Io(_) => f.write_str("could not send message"),
			SendError::Serialize(_) => f.write_str("could not serialize message"),
		}
	}
}

impl std::error::Error for SendError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			SendError::Io(err) => Some(err),
			SendError::Serialize(err) => Some(err),
		}
	}
}

/// An error from receiving a message using a [`Connection::recv`].
#[derive(Debug)]
pub enum RecvError {
	Deserialize(crate::proto::DeserializeError),
	Io(std::io::Error),
}

impl std::fmt::Display for RecvError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RecvError::Deserialize(_) => f.write_str("could not deserialize message"),
			RecvError::Io(_) => f.write_str("could not receive message"),
		}
	}
}

impl std::error::Error for RecvError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			RecvError::Deserialize(err) => Some(err),
			RecvError::Io(err) => Some(err),
		}
	}
}

fn connect(bus_address: &std::ffi::OsStr) -> Result<std::os::unix::net::UnixStream, ConnectError> {
	let bus_address_bytes = std::os::unix::ffi::OsStrExt::as_bytes(bus_address);

	let mut connect_errs = vec![];

	for bus_address_bytes in bus_address_bytes.split(|&b| b == b';') {
		if !bus_address_bytes.starts_with(b"unix:") {
			continue;
		}
		let bus_address_bytes = &bus_address_bytes[b"unix:".len()..];

		let path =
			bus_address_bytes.split(|&b| b == b',')
			.find_map(|pair| {
				let mut pair_parts = pair.splitn(2, |&b| b == b'=');

				let key = pair_parts.next().expect("split returns at least one subslice");
				if let Ok(key) = percent_encoding::percent_decode(key).decode_utf8() {
					if key == "path" {
						// We want to stop at the first `path` component even if it has no value,
						// so return `Some(None)` in that case rather than `None`.
						let value =
							pair_parts.next()
							.map(|value| {
								let value: Vec<u8> = percent_encoding::percent_decode(value).collect();
								let value: &std::ffi::OsStr = std::os::unix::ffi::OsStrExt::from_bytes(&value);
								let value: std::path::PathBuf = value.into();
								value
							});
						return Some(value);
					}
				}

				None
			});
		if let Some(Some(path)) = path {
			let stream = std::os::unix::net::UnixStream::connect(&path);
			match stream {
				Ok(stream) => return Ok(stream),
				Err(err) => connect_errs.push((path, err)),
			}
		}
	}

	Err(ConnectError::Connect(connect_errs))
}
