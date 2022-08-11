/// Message header.
#[derive(Debug)]
pub struct MessageHeader<'a> {
	/// The message type.
	pub r#type: MessageType<'a>,

	/// The message flags.
	pub flags: MessageFlags,

	/// The length of the message body, in bytes.
	pub body_len: usize,

	/// The message serial.
	pub serial: u32,

	/// Header fields.
	pub fields: std::borrow::Cow<'a, [MessageHeaderField<'a>]>,
}

pub fn deserialize_message<'a>(buf: &'a [u8]) -> Result<(MessageHeader<'a>, Option<crate::Variant<'a>>, usize), crate::DeserializeError> {
	// Arbitrarily pick `Endianness::Little` to initialize the deserializer. It'll be overridden as soon as the endianness marker is parsed.
	let mut deserializer = crate::de::Deserializer::new(buf, 0, crate::Endianness::Little);

	let EndiannessMarker(endianness) = EndiannessMarker::deserialize(&mut deserializer)?;
	deserializer.set_endianness(endianness);

	let message_header = MessageHeader::deserialize(&mut deserializer)?;

	deserializer.pad_to(8)?;

	let (message_body, read) =
		if message_header.body_len > 0 {
			let body_start_pos = deserializer.pos();
			let body_len = message_header.body_len;
			let body_end_pos = body_start_pos + body_len;

			if buf.len() < body_end_pos {
				return Err(crate::DeserializeError::EndOfInput);
			}

			let signature =
				message_header.fields.iter()
				.find_map(|message_header_field| match message_header_field {
					MessageHeaderField::Signature(signature) => Some(signature),
					_ => None,
				})
				.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "body-containing", header_field_name: "SIGNATURE" })?;

			let mut deserializer = crate::de::Deserializer::new(&buf[..body_end_pos], body_start_pos, endianness);

			let message_body = crate::Variant::deserialize(&mut deserializer, signature)?;

			(Some(message_body), body_end_pos)
		}
		else {
			(None, deserializer.pos())
		};

	Ok((message_header, message_body, read))
}

pub fn serialize_message(
	header: &mut MessageHeader<'_>,
	body: Option<&crate::Variant<'_>>,
	buf: &mut Vec<u8>,
	endianness: crate::Endianness,
) -> Result<(), crate::SerializeError> {
	let header_fields = header.fields.to_mut();

	match &mut header.r#type {
		MessageType::Error { name, reply_serial } => {
			header_fields.push(MessageHeaderField::ErrorName(std::mem::take(name)));
			header_fields.push(MessageHeaderField::ReplySerial(*reply_serial));
		},

		MessageType::MethodCall { member, path } => {
			header_fields.push(MessageHeaderField::Member(std::mem::take(member)));
			header_fields.push(MessageHeaderField::Path(std::mem::take(path)));
		},

		MessageType::MethodReturn { reply_serial } => {
			header_fields.push(MessageHeaderField::ReplySerial(*reply_serial));
		},

		MessageType::Signal { interface, member, path } => {
			header_fields.push(MessageHeaderField::Interface(std::mem::take(interface)));
			header_fields.push(MessageHeaderField::Member(std::mem::take(member)));
			header_fields.push(MessageHeaderField::Path(std::mem::take(path)));
		},
	}

	let body =
		if let Some(body) = body {
			let mut body_serialized = vec![];
			let mut body_serializer = crate::ser::Serializer::new(&mut body_serialized, endianness);
			body.serialize(&mut body_serializer)?;

			let body_len = body_serialized.len();

			let body_signature = body.inner_signature();

			Some((body_serialized, body_len, body_signature))
		}
		else {
			None
		};

	if let Some((body_serialized, body_len, body_signature)) = body {
		header.body_len = body_len;

		header_fields.push(MessageHeaderField::Signature(body_signature));

		let mut message_serializer = crate::ser::Serializer::new(buf, endianness);

		EndiannessMarker(endianness).serialize(&mut message_serializer);

		header.serialize(&mut message_serializer)?;

		message_serializer.pad_to(8);

		buf.extend_from_slice(&body_serialized);
	}
	else {
		let mut message_serializer = crate::ser::Serializer::new(buf, endianness);

		EndiannessMarker(endianness).serialize(&mut message_serializer);

		header.serialize(&mut message_serializer)?;

		message_serializer.pad_to(8);
	}

	Ok(())
}

impl<'de> MessageHeader<'de> {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'de>) -> Result<Self, crate::DeserializeError> {
		let r#type = deserializer.deserialize_u8()?;

		let flags = MessageFlags::deserialize(deserializer)?;

		let protocol_version = deserializer.deserialize_u8()?;
		if protocol_version != 0x01 {
			return Err(crate::DeserializeError::InvalidValue { expected: "0x01".into(), actual: protocol_version.to_string() });
		}

		let body_len = deserializer.deserialize_u32()?;
		let body_len: usize = body_len.try_into().map_err(crate::DeserializeError::ExceedsNumericLimits)?;

		let serial = deserializer.deserialize_u32()?;

		let fields = deserializer.deserialize_array(8, MessageHeaderField::deserialize)?;

		let (r#type, fields) = MessageType::from(r#type, fields)?;

		Ok(MessageHeader {
			r#type,
			flags,
			body_len,
			serial,
			fields: fields.into(),
		})
	}

	pub fn into_owned(self) -> MessageHeader<'static> {
		MessageHeader {
			r#type: self.r#type.into_owned(),
			flags: self.flags,
			body_len: self.body_len,
			serial: self.serial,
			fields: self.fields.iter().cloned().map(MessageHeaderField::into_owned).collect::<Vec<_>>().into(),
		}
	}
}

impl MessageHeader<'_> {
	fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		self.r#type.serialize(serializer);

		self.flags.serialize(serializer);

		serializer.serialize_u8(0x01_u8);

		crate::UsizeAsU32(self.body_len).serialize(serializer)?;

		serializer.serialize_u32(self.serial);

		serializer.serialize_array(
			1,
			&self.fields,
			MessageHeaderField::serialize,
		)?;

		Ok(())
	}
}

/// Message type.
#[derive(Clone, Debug)]
pub enum MessageType<'a> {
	Error {
		name: std::borrow::Cow<'a, str>,
		reply_serial: u32,
	},

	MethodCall {
		member: std::borrow::Cow<'a, str>,
		path: crate::ObjectPath<'a>,
	},

	MethodReturn {
		reply_serial: u32,
	},

	Signal {
		interface: std::borrow::Cow<'a, str>,
		member: std::borrow::Cow<'a, str>,
		path: crate::ObjectPath<'a>,
	},
}

impl<'a> MessageType<'a> {
	fn from(
		r#type: u8,
		fields: Vec<MessageHeaderField<'a>>,
	) -> Result<(Self, Vec<MessageHeaderField<'a>>), crate::DeserializeError> {
		// TODO: Use `Vec::drain_filter` when that stabilizes to mutate `fields` in place

		let mut other_fields = vec![];

		let mut error_name_field = None;
		let mut interface_field = None;
		let mut member_field = None;
		let mut path_field = None;
		let mut reply_serial_field = None;

		for field in fields {
			match field {
				MessageHeaderField::Destination(destination) =>
					other_fields.push(MessageHeaderField::Destination(destination)),

				MessageHeaderField::ErrorName(error_name) if r#type == 0x03 =>
					error_name_field = Some(error_name),
				MessageHeaderField::ErrorName(error_name) =>
					other_fields.push(MessageHeaderField::ErrorName(error_name)),

				MessageHeaderField::Interface(interface) if r#type == 0x04 =>
					interface_field = Some(interface),
				MessageHeaderField::Interface(interface) =>
					other_fields.push(MessageHeaderField::Interface(interface)),

				MessageHeaderField::Member(member) if r#type == 0x01 || r#type == 0x04 =>
					member_field = Some(member),
				MessageHeaderField::Member(member) =>
					other_fields.push(MessageHeaderField::Member(member)),

				MessageHeaderField::Path(path) if r#type == 0x01 || r#type == 0x04 =>
					path_field = Some(path),
				MessageHeaderField::Path(path) =>
					other_fields.push(MessageHeaderField::Path(path)),

				MessageHeaderField::ReplySerial(reply_serial) if r#type == 0x02 || r#type == 0x03 =>
					reply_serial_field = Some(reply_serial),
				MessageHeaderField::ReplySerial(reply_serial) =>
					other_fields.push(MessageHeaderField::ReplySerial(reply_serial)),

				MessageHeaderField::Sender(sender) =>
					other_fields.push(MessageHeaderField::Sender(sender)),

				MessageHeaderField::Signature(signature) =>
					other_fields.push(MessageHeaderField::Signature(signature)),

				MessageHeaderField::UnixFds(num_unix_fds) =>
					other_fields.push(MessageHeaderField::UnixFds(num_unix_fds)),

				MessageHeaderField::Unknown { code, value } =>
					other_fields.push(MessageHeaderField::Unknown { code, value }),
			}
		}

		let r#type = match r#type {
			0x01 => {
				let member =
					member_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "METHOD_CALL", header_field_name: "MEMBER" })?;
				let path =
					path_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "METHOD_CALL", header_field_name: "PATH" })?;
				MessageType::MethodCall {
					member,
					path,
				}
			},

			0x02 => {
				let reply_serial =
					reply_serial_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "METHOD_RETURN", header_field_name: "REPLY_SERIAL" })?;
				MessageType::MethodReturn {
					reply_serial,
				}
			},

			0x03 => {
				let name =
					error_name_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "ERROR", header_field_name: "NAME" })?;
				let reply_serial =
					reply_serial_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "ERROR", header_field_name: "REPLY_SERIAL" })?;
				MessageType::Error {
					name,
					reply_serial,
				}
			},

			0x04 => {
				let interface =
					interface_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "SIGNAL", header_field_name: "INTERFACE" })?;
				let member =
					member_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "SIGNAL", header_field_name: "MEMBER" })?;
				let path =
					path_field
					.ok_or(crate::DeserializeError::MissingRequiredMessageHeaderField { method_name: "SIGNAL", header_field_name: "PATH" })?;
				MessageType::Signal {
					interface,
					member,
					path,
				}
			},

			v => return Err(crate::DeserializeError::InvalidValue { expected: "one of 0x01, 0x02, 0x03, 0x04".into(), actual: v.to_string() }),
		};

		Ok((r#type, other_fields))
	}

	fn into_owned(self) -> MessageType<'static> {
		match self {
			MessageType::Error { name, reply_serial } => MessageType::Error {
				name: name.into_owned().into(),
				reply_serial,
			},

			MessageType::MethodCall { member, path } => MessageType::MethodCall {
				member: member.into_owned().into(),
				path: path.into_owned(),
			},

			MessageType::MethodReturn { reply_serial } => MessageType::MethodReturn {
				reply_serial,
			},

			MessageType::Signal { interface, member, path } => MessageType::Signal {
				interface: interface.into_owned().into(),
				member: member.into_owned().into(),
				path: path.into_owned(),
			},
		}
	}
}

impl MessageType<'_> {
	fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) {
		let r#type = match self {
			MessageType::Error { .. } => 0x03,
			MessageType::MethodCall { .. } => 0x01,
			MessageType::MethodReturn { .. } => 0x02,
			MessageType::Signal { .. } => 0x04,
		};
		serializer.serialize_u8(r#type);
	}
}

/// Message flags.
///
/// Bit-wise OR of the [`flags`] constants.
#[derive(Clone, Copy, Debug)]
pub struct MessageFlags(u8);

impl std::ops::BitOr for MessageFlags {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self {
		MessageFlags(self.0 | rhs.0)
	}
}

impl MessageFlags {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'_>) -> Result<Self, crate::DeserializeError> {
		Ok(MessageFlags(deserializer.deserialize_u8()?))
	}

	fn serialize(self, serializer: &mut crate::ser::Serializer<'_>) {
		serializer.serialize_u8(self.0);
	}
}

pub mod flags {
	pub const NONE: super::MessageFlags = super::MessageFlags(0x00);
	pub const NO_REPLY_EXPECTED: super::MessageFlags = super::MessageFlags(0x01);
	pub const NO_AUTO_START: super::MessageFlags = super::MessageFlags(0x02);
	pub const ALLOW_INTERACTIVE_AUTHORIZATION: super::MessageFlags = super::MessageFlags(0x04);
}

/// A message header field.
#[derive(Clone, Debug)]
pub enum MessageHeaderField<'a> {
	Destination(std::borrow::Cow<'a, str>),

	ErrorName(std::borrow::Cow<'a, str>),

	Interface(std::borrow::Cow<'a, str>),

	Member(std::borrow::Cow<'a, str>),

	Path(crate::ObjectPath<'a>),

	ReplySerial(u32),

	Sender(std::borrow::Cow<'a, str>),

	Signature(crate::Signature),

	UnixFds(u32),

	Unknown {
		code: u8,
		value: crate::Variant<'a>,
	},
}

impl<'de> MessageHeaderField<'de> {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'de>) -> Result<Self, crate::DeserializeError> {
		deserializer.deserialize_struct(|deserializer| {
			let code = deserializer.deserialize_u8()?;

			let signature = crate::Signature::deserialize(deserializer)?;
			let value = crate::Variant::deserialize(deserializer, &signature)?;

			#[allow(clippy::match_same_arms)]
			match (code, value) {
				(0x01, crate::Variant::ObjectPath(object_path)) =>
					Ok(MessageHeaderField::Path(object_path)),
				(0x01, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "an object path".into(), actual: format!("{value:?}") }),

				(0x02, crate::Variant::String(name)) =>
					Ok(MessageHeaderField::Interface(name)),
				(0x02, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x03, crate::Variant::String(name)) =>
					Ok(MessageHeaderField::Member(name)),
				(0x03, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x04, crate::Variant::String(name)) =>
					Ok(MessageHeaderField::ErrorName(name)),
				(0x04, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x05, crate::Variant::U32(serial)) =>
					Ok(MessageHeaderField::ReplySerial(serial)),
				(0x05, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x06, crate::Variant::String(name)) =>
					Ok(MessageHeaderField::Destination(name)),
				(0x06, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x07, crate::Variant::String(name)) =>
					Ok(MessageHeaderField::Sender(name)),
				(0x07, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a string".into(), actual: format!("{value:?}") }),

				(0x08, crate::Variant::Signature(signature)) =>
					Ok(MessageHeaderField::Signature(signature)),
				(0x08, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a signature".into(), actual: format!("{value:?}") }),

				(0x09, crate::Variant::U32(num_unix_fds)) =>
					Ok(MessageHeaderField::UnixFds(num_unix_fds)),
				(0x09, value) =>
					Err(crate::DeserializeError::InvalidValue { expected: "a u32".into(), actual: format!("{value:?}") }),

				(code, value) =>
					Ok(MessageHeaderField::Unknown { code, value }),
			}
		})
	}

	fn into_owned(self) -> MessageHeaderField<'static> {
		match self {
			MessageHeaderField::Destination(name) => MessageHeaderField::Destination(name.into_owned().into()),

			MessageHeaderField::ErrorName(name) => MessageHeaderField::ErrorName(name.into_owned().into()),

			MessageHeaderField::Interface(name) => MessageHeaderField::Interface(name.into_owned().into()),

			MessageHeaderField::Member(name) => MessageHeaderField::Member(name.into_owned().into()),

			MessageHeaderField::Path(object_path) => MessageHeaderField::Path(object_path.into_owned()),

			MessageHeaderField::ReplySerial(value) => MessageHeaderField::ReplySerial(value),

			MessageHeaderField::Sender(name) => MessageHeaderField::Sender(name.into_owned().into()),

			MessageHeaderField::Signature(signature) => MessageHeaderField::Signature(signature),

			MessageHeaderField::UnixFds(num_unix_fds) => MessageHeaderField::UnixFds(num_unix_fds),

			MessageHeaderField::Unknown { code, value } => MessageHeaderField::Unknown {
				code,
				value: value.into_owned(),
			},
		}
	}
}

impl MessageHeaderField<'_> {
	fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		let (code, value) = match self {
			MessageHeaderField::Destination(name) =>
				(0x06, std::borrow::Cow::Owned(crate::Variant::String(name.clone()))),

			MessageHeaderField::ErrorName(name) =>
				(0x04, std::borrow::Cow::Owned(crate::Variant::String(name.clone()))),

			MessageHeaderField::Interface(name) =>
				(0x02, std::borrow::Cow::Owned(crate::Variant::String(name.clone()))),

			MessageHeaderField::Member(name) =>
				(0x03, std::borrow::Cow::Owned(crate::Variant::String(name.clone()))),

			MessageHeaderField::Path(object_path) =>
				(0x01, std::borrow::Cow::Owned(crate::Variant::ObjectPath(object_path.clone()))),

			MessageHeaderField::ReplySerial(value) =>
				(0x05, std::borrow::Cow::Owned(crate::Variant::U32(*value))),

			MessageHeaderField::Sender(name) =>
				(0x07, std::borrow::Cow::Owned(crate::Variant::String(name.clone()))),

			MessageHeaderField::Signature(signature) =>
				(0x08, std::borrow::Cow::Owned(crate::Variant::Signature(signature.clone()))),

			MessageHeaderField::UnixFds(num_unix_fds) =>
				(0x09, std::borrow::Cow::Owned(crate::Variant::U32(*num_unix_fds))),

			MessageHeaderField::Unknown { code, value } =>
				(*code, std::borrow::Cow::Borrowed(value)),
		};

		serializer.serialize_struct(|serializer| {
			serializer.serialize_u8(code);

			let signature = value.inner_signature();
			signature.serialize(serializer)?;

			value.serialize(serializer)?;

			Ok(())
		})
	}
}

#[derive(Clone, Copy)]
struct EndiannessMarker(crate::Endianness);

impl EndiannessMarker {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'_>) -> Result<Self, crate::DeserializeError> {
		let endianness_marker = deserializer.deserialize_u8()?;
		let endianness = match endianness_marker {
			b'B' => crate::Endianness::Big,
			b'l' => crate::Endianness::Little,
			endianness_marker => return Err(crate::DeserializeError::InvalidValue { expected: "b'B' or b'l'".into(), actual: endianness_marker.to_string() }),
		};
		Ok(EndiannessMarker(endianness))
	}

	fn serialize(self, serializer: &mut crate::ser::Serializer<'_>) {
		let endianness_marker = match self.0 {
			crate::Endianness::Big => b'B',
			crate::Endianness::Little => b'l',
		};
		serializer.serialize_u8(endianness_marker);
	}
}
