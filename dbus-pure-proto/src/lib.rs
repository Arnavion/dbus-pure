#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::let_unit_value,
	clippy::missing_errors_doc,
	clippy::module_name_repetitions,
	clippy::must_use_candidate,
	clippy::needless_lifetimes,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unneeded_field_pattern,
)]

//! This is a pure Rust implementation of the D-Bus binary protocol.
//!
//! Use [`deserialize_message`] to parse a D-Bus message from raw bytes, and [`serialize_message`] to convert a D-Bus message to raw bytes.
//!
//! To actually connect to a bus and communicate with it, see the `dbus-pure` crate.

mod as_variant;
pub use as_variant::{
	AsVariant,
};

pub(crate) mod de;
pub use de::{
	DeserializeError,
};

pub(crate) mod message;
pub use message::{
	deserialize_message,
	flags as message_flags,
	MessageFlags,
	MessageHeader,
	MessageHeaderField,
	MessageType,
	serialize_message,
};

pub(crate) mod ser;
pub use ser::{
	SerializeError,
};

pub mod std2;

mod variant;
pub use variant::{
	Variant,
};

mod variant_deserializer;
pub use variant_deserializer::{
	VariantDeserializeError,
};

#[derive(Clone, Copy, Debug)]
pub enum Endianness {
	Big,
	Little,
}

macro_rules! endianness_from_bytes {
	($($fn:ident -> $ty:ty,)*) => {
		impl Endianness {
			$(
				fn $fn(self, bytes: [u8; std::mem::size_of::<$ty>()]) -> $ty {
					match self {
						Endianness::Big => <$ty>::from_be_bytes(bytes),
						Endianness::Little => <$ty>::from_le_bytes(bytes),
					}
				}
			)*
		}
	};
}

endianness_from_bytes! {
	i16_from_bytes -> i16,
	i32_from_bytes -> i32,
	i64_from_bytes -> i64,

	u16_from_bytes -> u16,
	u32_from_bytes -> u32,
	u64_from_bytes -> u64,

	f64_from_bytes -> f64,
}


macro_rules! endianness_to_bytes {
	($($fn:ident -> $ty:ty,)*) => {
		impl Endianness {
			$(
				fn $fn(self, value: $ty) -> [u8; std::mem::size_of::<$ty>()] {
					match self {
						Endianness::Big => <$ty>::to_be_bytes(value),
						Endianness::Little => <$ty>::to_le_bytes(value),
					}
				}
			)*
		}
	};
}

endianness_to_bytes! {
	i16_to_bytes -> i16,
	i32_to_bytes -> i32,
	i64_to_bytes -> i64,

	u16_to_bytes -> u16,
	u32_to_bytes -> u32,
	u64_to_bytes -> u64,

	f64_to_bytes -> f64,
}

/// An object path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObjectPath<'a>(pub std::borrow::Cow<'a, str>);

impl<'de> ObjectPath<'de> {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'de>) -> Result<Self, crate::DeserializeError> {
		Ok(ObjectPath(deserializer.deserialize_string()?.into()))
	}

	fn into_owned(self) -> ObjectPath<'static> {
		ObjectPath(self.0.into_owned().into())
	}
}

impl ObjectPath<'_> {
	fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		serializer.serialize_string(&self.0)
	}
}

/// A signature.
///
/// Use `.to_string()` to get the string representation of the signature.
#[derive(Clone, Debug, Eq, PartialEq)]
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
				write!(f, "a{element}")?,

			Signature::Bool =>
				f.write_str("b")?,

			Signature::DictEntry { key, value } => {
				f.write_str("{")?;
				write!(f, "{key}")?;
				write!(f, "{value}")?;
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
					write!(f, "{field}")?;
				}
				f.write_str(")")?;
			},

			Signature::Tuple { elements } =>
				for element in elements {
					write!(f, "{element}")?;
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

						let field = from_inner(chars)?;
						fields.push(field);
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
		if chars.peek().is_none() {
			return Ok(Signature::Tuple { elements: vec![] });
		}

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

impl Signature {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'_>) -> Result<Self, crate::DeserializeError> {
		let len = deserializer.deserialize_u8()?;

		let mut signature = String::with_capacity(len.into());
		for _ in 0..len {
			let b = deserializer.deserialize_u8()?;
			signature.push(b as char);
		}

		let nul = deserializer.deserialize_u8()?;
		if nul != b'\0' {
			return Err(crate::DeserializeError::InvalidValue { expected: "0x00".into(), actual: nul.to_string() });
		}

		let signature =
			signature.parse()
			.map_err(|()| crate::DeserializeError::InvalidValue { expected: "a signature".into(), actual: signature })?;
		Ok(signature)
	}

	fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		let signature_string = self.to_string();

		let len: u8 = signature_string.len().try_into().map_err(crate::SerializeError::ExceedsNumericLimits)?;

		let data = std::iter::once(len).chain(signature_string.as_bytes().iter().copied()).chain(std::iter::once(b'\0'));

		for b in data {
			serializer.serialize_u8(b);
		}

		Ok(())
	}
}

/// An index into an array of file descriptors.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnixFd(pub u32);

impl UnixFd {
	fn deserialize(deserializer: &mut crate::de::Deserializer<'_>) -> Result<Self, crate::DeserializeError> {
		Ok(UnixFd(deserializer.deserialize_u32()?))
	}

	fn serialize(self, serializer: &mut crate::ser::Serializer<'_>) {
		serializer.serialize_u32(self.0);
	}
}

/// A trait representing a generic object on a message bus.
pub trait Object {
	fn name(&self) -> &str;
	fn path(&self) -> ObjectPath<'_>;
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct UsizeAsU32(pub(crate) usize);

impl UsizeAsU32 {
	fn serialize(self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		let value: u32 = self.0.try_into().map_err(crate::SerializeError::ExceedsNumericLimits)?;
		serializer.serialize_u32(value);
		Ok(())
	}
}
