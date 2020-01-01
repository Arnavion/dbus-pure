/// A connection to a message bus.
pub struct Connection {
	reader: std::io::BufReader<std::os::unix::net::UnixStream>,
	read_buf: Vec<u8>,
	read_end: usize,
	writer: std::os::unix::net::UnixStream,
	write_buf: Vec<u8>,
	server_guid: Vec<u8>,
}

/// The path of a message bus.
#[derive(Clone, Copy, Debug)]
pub enum BusPath<'a> {
	/// The session bus. Its path will be determined from the `DBUS_SESSION_BUS_ADDRESS` environment variable.
	Session,

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
				let session_bus_address = std::env::var_os("DBUS_SESSION_BUS_ADDRESS").ok_or_else(|| ConnectError::SessionBusEnvVar(None))?;
				let bus_path: &std::ffi::OsStr = {
					let session_bus_address_bytes = std::os::unix::ffi::OsStrExt::as_bytes(&*session_bus_address);
					if session_bus_address_bytes.starts_with(b"unix:path=") {
						std::os::unix::ffi::OsStrExt::from_bytes(&session_bus_address_bytes["unix:path=".len()..])
					}
					else {
						return Err(ConnectError::SessionBusEnvVar(Some(session_bus_address)));
					}
				};
				let bus_path = std::path::Path::new(bus_path);
				let stream =
					std::os::unix::net::UnixStream::connect(bus_path)
					.map_err(|err| ConnectError::Connect { bus_path: bus_path.to_owned(), err, })?;
				stream
			},

			BusPath::UnixSocketFile(bus_path) => {
				let stream =
					std::os::unix::net::UnixStream::connect(bus_path)
					.map_err(|err| ConnectError::Connect { bus_path: bus_path.to_owned(), err, })?;
				stream
			},
		};

		let sasl_auth_id: std::borrow::Cow<'_, str> = match sasl_auth_type {
			SaslAuthType::Uid =>
				(unsafe { libc::getuid() })
				.to_string()
				.chars()
				.map(|c| format!("{:2x}", c as u32))
				.collect::<String>()
				.into(),

			SaslAuthType::Other(sasl_auth_id) => sasl_auth_id.into(),
		};

		let reader = stream.try_clone().map_err(ConnectError::Authenticate)?;
		let mut reader = std::io::BufReader::new(reader);
		let mut read_buf = vec![];

		let mut writer = stream;
		let write_buf = vec![];

		#[allow(clippy::write_with_newline)]
		write!(writer, "\0AUTH EXTERNAL {}\r\n", sasl_auth_id).map_err(ConnectError::Authenticate)?;
		writer.flush().map_err(ConnectError::Authenticate)?;

		let _ = reader.read_until(b'\n', &mut read_buf).map_err(ConnectError::Authenticate)?;
		if read_buf.iter().rev().nth(1).copied() != Some(b'\r') {
			return Err(ConnectError::Authenticate(std::io::Error::new(std::io::ErrorKind::Other, "malformed response")));
		}

		let server_guid =
			if read_buf.starts_with(b"OK ") {
				&read_buf[b"OK ".len()..(b"OK ".len() + 32)]
			}
			else {
				return Err(ConnectError::Authenticate(std::io::Error::new(std::io::ErrorKind::Other, "malformed response")));
			};
		let server_guid = server_guid.to_owned();

		read_buf.clear();
		read_buf.resize(1, 0);

		writer.write_all(b"BEGIN\r\n").map_err(ConnectError::Authenticate)?;
		writer.flush().map_err(ConnectError::Authenticate)?;

		Ok(Connection {
			reader,
			read_buf,
			read_end: 0,
			writer,
			write_buf,
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
	pub fn send(&mut self, header: &mut crate::types::MessageHeader<'_>, body: Option<&crate::types::Variant<'_>>) -> Result<(), SendError> {
		use std::io::Write;

		let () = crate::types::message::serialize_message(header, body, &mut self.write_buf).map_err(SendError::Serialize)?;

		let _ = self.writer.write_all(&self.write_buf).map_err(SendError::Io)?;
		self.write_buf.clear();

		let () = self.writer.flush().map_err(SendError::Io)?;

		Ok(())
	}

	/// Receive a message from the message bus.
	pub fn recv(&mut self) -> Result<(crate::types::MessageHeader<'static>, Option<crate::types::Variant<'static>>), RecvError> {
		use std::io::Read;

		loop {
			match crate::types::message::deserialize_message(&self.read_buf[..self.read_end]) {
				Ok((message_header, message_body, read)) => {
					self.read_buf.copy_within(read..self.read_end, 0);
					self.read_end -= read;
					return Ok((message_header, message_body));
				},

				Err(crate::de::DeserializeError::EndOfInput) => {
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
}

/// An error from connecting to a message bus.
#[derive(Debug)]
pub enum ConnectError {
	Authenticate(std::io::Error),

	Connect {
		bus_path: std::path::PathBuf,
		err: std::io::Error,
	},

	SessionBusEnvVar(Option<std::ffi::OsString>),
}

impl std::fmt::Display for ConnectError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConnectError::Authenticate(_) => f.write_str("could not authenticate with bus"),
			ConnectError::Connect { bus_path, err: _ } => write!(f, "could not connect to bus path {}", bus_path.display()),
			ConnectError::SessionBusEnvVar(None) => f.write_str("the DBUS_SESSION_BUS_ADDRESS env var is not set"),
			ConnectError::SessionBusEnvVar(Some(value)) => write!(f, "the DBUS_SESSION_BUS_ADDRESS env var is malformed: {:?}", value),
		}
	}
}

impl std::error::Error for ConnectError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		#[allow(clippy::match_same_arms)]
		match self {
			ConnectError::Authenticate(err) => Some(err),
			ConnectError::Connect { bus_path: _, err } => Some(err),
			ConnectError::SessionBusEnvVar(_) => None,
		}
	}
}

/// An error from sending a message using a [`Connection::send`].
#[derive(Debug)]
pub enum SendError {
	Io(std::io::Error),
	Serialize(crate::ser::SerializeError),
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
	Deserialize(crate::de::DeserializeError),
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
