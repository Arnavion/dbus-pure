/// D-Bus built-in types and message types.

pub(crate) mod message;
pub use message::{
	MessageFlags,
	MessageHeader,
	MessageHeaderField,
	MessageType,
	flags as message_flags,
};

mod variant;
pub use variant::{
	Variant,
	VariantDeserializeSeed,
};

/// An object path.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectPath<'a>(pub std::borrow::Cow<'a, str>);

impl serde::Serialize for ObjectPath<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		self.0.serialize(serializer)
	}
}

impl<'de, 'a> serde::Deserialize<'de> for ObjectPath<'a> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		Ok(ObjectPath(serde::de::Deserialize::deserialize(deserializer)?))
	}
}

/// A signature.
///
/// Use `.to_string()` to get the string representation of the signature.
#[derive(Clone, Debug, PartialEq)]
pub enum Signature {
	Array { element: Box<Signature> },
	Bool,
	DictEntry { key: Box<Signature>, value: Box<Signature> },
	F64,
	I16,
	I32,
	I64,
	ObjectPath,
	Signature,
	String,
	Struct { fields: Vec<Signature> },
	Tuple { elements: Vec<Signature> },
	U8,
	U16,
	U32,
	U64,
	UnixFd,
	Variant,
}

impl Signature {
	fn alignment(&self) -> usize {
		#[allow(clippy::match_same_arms)]
		match self {
			Signature::Array { .. } => 4,
			Signature::Bool => 4,
			Signature::DictEntry { .. } => 8,
			Signature::F64 => 8,
			Signature::I16 => 2,
			Signature::I32 => 4,
			Signature::I64 => 8,
			Signature::ObjectPath => 4,
			Signature::Signature => 1,
			Signature::String => 4,
			Signature::Struct { .. } => 8,
			Signature::Tuple { .. } => 1,
			Signature::U8 => 1,
			Signature::U16 => 2,
			Signature::U32 => 4,
			Signature::U64 => 8,
			Signature::UnixFd => 4,
			Signature::Variant => 1,
		}
	}
}

impl std::fmt::Display for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Signature::Array { element } =>
				write!(f, "a{}", element)?,

			Signature::Bool =>
				f.write_str("b")?,

			Signature::DictEntry { key, value } => {
				f.write_str("{")?;
				write!(f, "{}", key)?;
				write!(f, "{}", value)?;
				f.write_str("}")?;
			},

			Signature::F64 =>
				f.write_str("d")?,

			Signature::I16 =>
				f.write_str("n")?,

			Signature::I32 =>
				f.write_str("i")?,

			Signature::I64 =>
				f.write_str("x")?,

			Signature::ObjectPath =>
				f.write_str("o")?,

			Signature::Signature =>
				f.write_str("g")?,

			Signature::String =>
				f.write_str("s")?,

			Signature::Struct { fields } => {
				f.write_str("(")?;
				for field in fields {
					write!(f, "{}", field)?;
				}
				f.write_str(")")?;
			},

			Signature::Tuple { elements } =>
				for element in elements {
					write!(f, "{}", element)?;
				},

			Signature::U8 =>
				f.write_str("y")?,

			Signature::U16 =>
				f.write_str("q")?,

			Signature::U32 =>
				f.write_str("u")?,

			Signature::U64 =>
				f.write_str("t")?,

			Signature::UnixFd =>
				f.write_str("h")?,

			Signature::Variant =>
				f.write_str("v")?,
		}

		Ok(())
	}
}

impl std::str::FromStr for Signature {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn from_inner(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<Signature, ()> {
			match chars.next().ok_or(())? {
				'a' => {
					let element = from_inner(chars)?;
					Ok(Signature::Array { element: Box::new(element) })
				},

				'b' => Ok(Signature::Bool),

				'd' => Ok(Signature::F64),

				'g' => Ok(Signature::Signature),

				'h' => Ok(Signature::UnixFd),

				'i' => Ok(Signature::I32),

				'n' => Ok(Signature::I16),

				'o' => Ok(Signature::ObjectPath),

				'q' => Ok(Signature::U16),

				's' => Ok(Signature::String),

				't' => Ok(Signature::U64),

				'u' => Ok(Signature::U32),

				'v' => Ok(Signature::Variant),

				'x' => Ok(Signature::I64),

				'y' => Ok(Signature::U8),

				'(' => {
					let mut fields = vec![];

					loop {
						let next = chars.peek().copied();
						if next == Some(')') {
							let _ = chars.next();
							break;
						}
						else {
							let field = from_inner(chars)?;
							fields.push(field);
						}
					}

					Ok(Signature::Struct { fields })
				},

				'{' => {
					let key = from_inner(chars)?;

					let value = from_inner(chars)?;

					let next = chars.next();
					if next != Some('}') {
						return Err(());
					}

					Ok(Signature::DictEntry { key: Box::new(key), value: Box::new(value) })
				},

				_ => Err(()),
			}
		}

		let mut chars = s.chars().peekable();

		let first = from_inner(&mut chars)?;
		if chars.peek().is_some() {
			let mut elements = vec![first];
			while chars.peek().is_some() {
				elements.push(from_inner(&mut chars)?);
			}
			Ok(Signature::Tuple { elements })
		}
		else {
			Ok(first)
		}
	}
}

impl<'de> serde::Deserialize<'de> for Signature {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor;

		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Signature;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("signature")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
				let len: u8 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("signature"))?;

				let mut signature = String::with_capacity(len.into());
				for _ in 0..len {
					let b: u8 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("signature"))?;
					signature.push(b as char);
				}

				let nul: u8 = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("signature"))?;
				if nul != b'\0' {
					return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(nul.into()), &"0x00"));
				}

				let signature =
					signature.parse()
					.map_err(|()| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&signature), &self))?;
				Ok(signature)
			}
		}

		deserializer.deserialize_tuple(0, Visitor)
	}
}

impl serde::Serialize for Signature {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		use serde::ser::SerializeTuple;

		let signature_string = self.to_string();

		let len: Result<u8, _> = std::convert::TryInto::try_into(signature_string.len());
		let len = len.map_err(serde::ser::Error::custom)?;

		let data = std::iter::once(len).chain(signature_string.as_bytes().iter().copied()).chain(std::iter::once(b'\0'));

		let mut serializer = serializer.serialize_tuple(0)?;
		for b in data {
			serializer.serialize_element(&b)?;
		}
		serializer.end()
	}
}

/// An index into an array of file descriptors.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnixFd(pub u32);

impl serde::Serialize for UnixFd {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		self.0.serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for UnixFd {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		Ok(UnixFd(serde::de::Deserialize::deserialize(deserializer)?))
	}
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct UsizeAsU32(pub(crate) usize);

impl serde::Serialize for UsizeAsU32 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let value: u32 = std::convert::TryInto::try_into(self.0).map_err(serde::ser::Error::custom)?;
		value.serialize(serializer)
	}
}

pub(crate) struct Slice<'a, T> {
	pub(crate) inner: &'a [T],
	pub(crate) alignment: usize,
}

impl<T> serde::Serialize for Slice<'_, T> where T: serde::Serialize {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		use serde::ser::SerializeSeq;

		let mut serializer = serializer.serialize_seq(Some(self.alignment))?;
		for element in self.inner {
			serializer.serialize_element(element)?;
		}
		serializer.end()
	}
}
