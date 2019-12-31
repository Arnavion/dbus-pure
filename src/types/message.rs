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

pub(crate) fn deserialize_message(buf: &[u8]) -> Result<(MessageHeader<'static>, Option<crate::types::Variant<'static>>, usize), crate::de::DeserializeError> {
	let mut deserializer = crate::de::Deserializer::new(buf, 0);

	let mut message_header: MessageHeader<'static> = serde::Deserialize::deserialize(&mut deserializer)?;

	deserializer.pad_to(8)?;

	let (message_body, read) =
		if message_header.body_len > 0 {
			let body_start_pos = deserializer.pos();
			let body_len = message_header.body_len;
			let body_end_pos = body_start_pos + body_len;

			if buf.len() < body_end_pos {
				return Err(crate::de::DeserializeError::EndOfInput);
			}

			let signature =
				message_header.fields.iter()
				.find_map(|message_header_field| match message_header_field {
					MessageHeaderField::Signature(signature) => Some(signature),
					_ => None,
				})
				.ok_or_else(|| serde::de::Error::custom("message has non-empty body but not signature field in its header"))?;
			let deserialize_seed = crate::types::VariantDeserializeSeed::new(signature);

			let mut deserializer = crate::de::Deserializer::new(&buf[..body_end_pos], body_start_pos);

			let message_body: crate::types::Variant<'static> = serde::de::DeserializeSeed::deserialize(deserialize_seed, &mut deserializer)?;

			(Some(message_body), body_end_pos)
		}
		else {
			(None, deserializer.pos())
		};

	match message_header.r#type {
		MessageType::Error { .. } => {
			let name =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::ErrorName(name) = field { Some(name.clone()) } else { None });

			let reply_serial =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::ReplySerial(serial) = field { Some(*serial) } else { None });

			message_header.r#type = match (name, reply_serial) {
				(Some(name), Some(reply_serial)) => MessageType::Error {
					name,
					reply_serial,
				},

				(None, _) => return Err(serde::de::Error::custom("ERROR message does not have ERROR_NAME header field")),

				(_, None) => return Err(serde::de::Error::custom("ERROR message does not have REPLY_SERIAL header field")),
			};
		},

		MessageType::MethodCall { .. } => {
			let member =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::Member(member) = field { Some(member.clone()) } else { None });

			let path =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::Path(path) = field { Some(path.clone()) } else { None });

			message_header.r#type = match (member, path) {
				(Some(member), Some(path)) => MessageType::MethodCall {
					member,
					path,
				},

				(None, _) => return Err(serde::de::Error::custom("METHOD_CALL message does not have MEMBER header field")),

				(_, None) => return Err(serde::de::Error::custom("METHOD_CALL message does not have PATH header field")),
			};
		},

		MessageType::MethodReturn { .. } => {
			let reply_serial =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::ReplySerial(serial) = field { Some(*serial) } else { None });

			message_header.r#type = match reply_serial {
				Some(reply_serial) => MessageType::MethodReturn {
					reply_serial,
				},

				None => return Err(serde::de::Error::custom("METHOD_RETURN message does not have REPLY_SERIAL header field")),
			};
		},

		MessageType::Signal { .. } => {
			let interface =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::Interface(interface) = field { Some(interface.clone()) } else { None });

			let member =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::Member(member) = field { Some(member.clone()) } else { None });

			let path =
				message_header.fields.iter()
				.find_map(|field| if let MessageHeaderField::Path(path) = field { Some(path.clone()) } else { None });

			message_header.r#type = match (interface, member, path) {
				(Some(interface), Some(member), Some(path)) => MessageType::Signal {
					interface,
					member,
					path,
				},

				(None, _, _) => return Err(serde::de::Error::custom("SIGNAL message does not have INTERFACE header field")),

				(_, None, _) => return Err(serde::de::Error::custom("SIGNAL message does not have MEMBER header field")),

				(_, _, None) => return Err(serde::de::Error::custom("SIGNAL message does not have PATH header field")),
			};
		},
	}

	Ok((message_header, message_body, read))
}

pub(crate) fn serialize_message(
	header: &mut MessageHeader<'_>,
	body: Option<&crate::types::Variant<'_>>,
	buf: &mut Vec<u8>,
) -> Result<(), crate::ser::SerializeError> {
	use serde::Serialize;

	let header_fields = header.fields.to_mut();

	match &header.r#type {
		MessageType::Error { name, reply_serial } => {
			header_fields.push(MessageHeaderField::ErrorName(name.clone()));
			header_fields.push(MessageHeaderField::ReplySerial(*reply_serial));
		},

		MessageType::MethodCall { member, path } => {
			header_fields.push(MessageHeaderField::Member(member.clone()));
			header_fields.push(MessageHeaderField::Path(path.clone()));
		},

		MessageType::MethodReturn { reply_serial } => {
			header_fields.push(MessageHeaderField::ReplySerial(*reply_serial));
		},

		MessageType::Signal { interface, member, path } => {
			header_fields.push(MessageHeaderField::Interface(interface.clone()));
			header_fields.push(MessageHeaderField::Member(member.clone()));
			header_fields.push(MessageHeaderField::Path(path.clone()));
		},
	}

	let body =
		if let Some(body) = body {
			let mut body_serialized = vec![];
			let mut body_serializer = crate::ser::Serializer::new(&mut body_serialized);
			body.serialize(&mut body_serializer)?;
			drop(body_serializer);

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

		let mut message_serializer = crate::ser::Serializer::new(buf);
		header.serialize(&mut message_serializer)?;
		message_serializer.pad_to(8);

		buf.extend_from_slice(&body_serialized);
	}
	else {
		let mut message_serializer = crate::ser::Serializer::new(buf);
		header.serialize(&mut message_serializer)?;
		message_serializer.pad_to(8);
	}

	Ok(())
}

impl<'de> serde::Deserialize<'de> for MessageHeader<'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = MessageHeader<'static>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("message header")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
				let endianness: u8 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("endianness"))?;
				if endianness != b'l' {
					return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(endianness.into()), &"'l'"));
				}

				let r#type: MessageType<'static> = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("type"))?;

				let flags: MessageFlags = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("flags"))?;

				let protocol_version: u8 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("protocol_version"))?;
				if protocol_version != 0x01 {
					return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(protocol_version.into()), &"0x01"));
				}

				let body_len: u32 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("body_len"))?;
				let body_len: usize = std::convert::TryInto::try_into(body_len).map_err(serde::de::Error::custom)?;

				let serial: u32 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("serial"))?;

				let fields: Vec<MessageHeaderField<'static>> = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("fields"))?;

				Ok(MessageHeader {
					r#type,
					flags,
					body_len,
					serial,
					fields: fields.into(),
				})
			}
		}

		deserializer.deserialize_tuple(7, Visitor)
	}
}

impl serde::Serialize for MessageHeader<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		use serde::ser::SerializeTuple;

		let mut serializer = serializer.serialize_tuple(7)?;

		serializer.serialize_element(&b'l')?;

		serializer.serialize_element(&self.r#type)?;

		serializer.serialize_element(&self.flags)?;

		serializer.serialize_element(&0x01_u8)?;

		serializer.serialize_element(&crate::types::UsizeAsU32(self.body_len))?;

		serializer.serialize_element(&self.serial)?;

		serializer.serialize_element(&self.fields)?;

		serializer.end()
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
		path: crate::types::ObjectPath<'a>,
	},

	MethodReturn {
		reply_serial: u32,
	},

	Signal {
		interface: std::borrow::Cow<'a, str>,
		member: std::borrow::Cow<'a, str>,
		path: crate::types::ObjectPath<'a>,
	},
}

impl<'de> serde::Deserialize<'de> for MessageType<'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = MessageType<'static>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("message type")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: serde::de::Error {
				match v {
					0x01 => Ok(MessageType::MethodCall {
						member: Default::default(),
						path: Default::default(),
					}),
					0x02 => Ok(MessageType::MethodReturn {
						reply_serial: Default::default(),
					}),
					0x03 => Ok(MessageType::Error {
						name: Default::default(),
						reply_serial: Default::default(),
					}),
					0x04 => Ok(MessageType::Signal {
						interface: Default::default(),
						member: Default::default(),
						path: Default::default(),
					}),
					v => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &"one of 0x01, 0x02, 0x03, 0x04")),
				}
			}
		}

		deserializer.deserialize_u8(Visitor)
	}
}

impl serde::Serialize for MessageType<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let r#type: u8 = match self {
			MessageType::Error { .. } => 0x03,
			MessageType::MethodCall { .. } => 0x01,
			MessageType::MethodReturn { .. } => 0x02,
			MessageType::Signal { .. } => 0x04,
		};
		r#type.serialize(serializer)
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

impl<'de> serde::Deserialize<'de> for MessageFlags {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = MessageFlags;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("message flags")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> {
				Ok(MessageFlags(v))
			}
		}

		deserializer.deserialize_u8(Visitor)
	}
}

impl serde::Serialize for MessageFlags {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		self.0.serialize(serializer)
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

	Path(crate::types::ObjectPath<'a>),

	ReplySerial(u32),

	Sender(std::borrow::Cow<'a, str>),

	Signature(crate::types::Signature),

	// UnixFds(u32),

	Unknown {
		code: u8,
		value: crate::types::Variant<'a>,
	},
}

impl<'de> serde::Deserialize<'de> for MessageHeaderField<'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = MessageHeaderField<'static>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("message header field")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
				// TODO: Any way to make this symmetric with the Serialize impl?
				//
				// TODO: This currently reads *two* values for every key, so it's likely to explode with any third-party deserializer.

				let code: u8 = map.next_key()?.ok_or_else(|| serde::de::Error::missing_field("code"))?;

				let signature: crate::types::Signature = map.next_value()?;
				let seed = crate::types::VariantDeserializeSeed::new(&signature);
				let value: crate::types::Variant<'static> = map.next_value_seed(seed)?;

				#[allow(clippy::match_same_arms)]
				match (code, value) {
					(0x01, crate::types::Variant::ObjectPath(object_path)) =>
						Ok(MessageHeaderField::Path(object_path)),
					(0x01, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"object path"))
					},

					(0x02, crate::types::Variant::String(name)) =>
						Ok(MessageHeaderField::Interface(name)),
					(0x02, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"string"))
					},

					(0x03, crate::types::Variant::String(name)) =>
						Ok(MessageHeaderField::Member(name)),
					(0x03, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"string"))
					},

					(0x04, crate::types::Variant::String(name)) =>
						Ok(MessageHeaderField::ErrorName(name)),
					(0x04, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"string"))
					},

					(0x05, crate::types::Variant::U32(serial)) =>
						Ok(MessageHeaderField::ReplySerial(serial)),
					(0x05, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"serial"))
					},

					(0x06, crate::types::Variant::String(name)) =>
						Ok(MessageHeaderField::Destination(name)),
					(0x06, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"string"))
					},

					(0x07, crate::types::Variant::String(name)) =>
						Ok(MessageHeaderField::Sender(name)),
					(0x07, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"string"))
					},

					(0x08, crate::types::Variant::Signature(signature)) =>
						Ok(MessageHeaderField::Signature(signature)),
					(0x08, value) => {
						let unexpected = format!("{:?}", value);
						Err(serde::de::Error::invalid_value(serde::de::Unexpected::Other(&unexpected), &"signature"))
					},

					(code, value) =>
						Ok(MessageHeaderField::Unknown { code, value }),
				}
			}
		}

		deserializer.deserialize_struct("MessageHeaderField", &["code", "signature", "value"], Visitor)
	}
}

impl serde::Serialize for MessageHeaderField<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		use serde::ser::SerializeStruct;

		let (code, value) = match self {
			MessageHeaderField::Destination(name) =>
				(0x06_u8, std::borrow::Cow::Owned(crate::types::Variant::String(name.clone()))),

			MessageHeaderField::ErrorName(name) =>
				(0x04_u8, std::borrow::Cow::Owned(crate::types::Variant::String(name.clone()))),

			MessageHeaderField::Interface(name) =>
				(0x02_u8, std::borrow::Cow::Owned(crate::types::Variant::String(name.clone()))),

			MessageHeaderField::Member(name) =>
				(0x03_u8, std::borrow::Cow::Owned(crate::types::Variant::String(name.clone()))),

			MessageHeaderField::Path(object_path) =>
				(0x01_u8, std::borrow::Cow::Owned(crate::types::Variant::ObjectPath(object_path.clone()))),

			MessageHeaderField::ReplySerial(value) =>
				(0x05_u8, std::borrow::Cow::Owned(crate::types::Variant::U32(*value))),

			MessageHeaderField::Sender(name) =>
				(0x07_u8, std::borrow::Cow::Owned(crate::types::Variant::String(name.clone()))),

			MessageHeaderField::Signature(signature) =>
				(0x08_u8, std::borrow::Cow::Owned(crate::types::Variant::Signature(signature.clone()))),

			MessageHeaderField::Unknown { code, value } =>
				(*code, std::borrow::Cow::Borrowed(value)),
		};

		let mut serializer = serializer.serialize_struct("MessageHeaderField", 2)?;

		serializer.serialize_field("code", &code)?;

		let signature = value.inner_signature();
		serializer.serialize_field("signature", &signature)?;

		serializer.serialize_field("value", &value)?;

		serializer.end()
	}
}
