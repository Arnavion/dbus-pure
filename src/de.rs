#[derive(Debug)]
pub(crate) struct Deserializer<'de> {
	buf: &'de [u8],
	pos: usize,
	endianness: crate::Endianness,
}

impl<'de> Deserializer<'de> {
	pub(crate) fn new(buf: &'de [u8], pos: usize, endianness: crate::Endianness) -> Self {
		Deserializer {
			buf,
			pos,
			endianness,
		}
	}

	pub(crate) fn pad_to(&mut self, alignment: usize) -> Result<(), DeserializeError> {
		let new_pos = ((self.pos + alignment - 1) / alignment) * alignment;
		if self.buf.len() < new_pos {
			return Err(DeserializeError::EndOfInput);
		}

		for &b in &self.buf[self.pos..new_pos] {
			if b != 0x00 {
				return Err(DeserializeError::NonZeroPadding);
			}
		}

		self.pos = new_pos;

		Ok(())
	}

	pub(crate) fn pos(&self) -> usize {
		self.pos
	}

	pub(crate) fn set_endianness(&mut self, endianness: crate::Endianness) {
		self.endianness = endianness;
	}
}

impl<'de> serde::Deserializer<'de> for &'_ mut Deserializer<'de> {
	type Error = DeserializeError;

	fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		Err(DeserializeError::DeserializeAnyNotSupported)
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 4)];
		self.pos += 4;

		let value: &[_; 4] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.u32_from_bytes(*value);
		match value {
			0 => visitor.visit_bool(false),
			1 => visitor.visit_bool(true),
			value => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(value.into()), &"0 or 1")),
		}
	}

	fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(2)?;

		if self.buf.len() < self.pos + 2 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 2)];
		self.pos += 2;

		let value: &[_; 2] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.i16_from_bytes(*value);
		visitor.visit_i16(value)
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 4)];
		self.pos += 4;

		let value: &[_; 4] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.i32_from_bytes(*value);
		visitor.visit_i32(value)
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 8)];
		self.pos += 8;

		let value: &[_; 8] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.i64_from_bytes(*value);
		visitor.visit_i64(value)
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		if self.buf.len() < self.pos + 1 {
			return Err(DeserializeError::EndOfInput);
		}

		let value = self.buf[self.pos];
		self.pos += 1;

		visitor.visit_u8(value)
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(2)?;

		if self.buf.len() < self.pos + 2 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 2)];
		self.pos += 2;

		let value: &[_; 2] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.u16_from_bytes(*value);
		visitor.visit_u16(value)
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 4)];
		self.pos += 4;

		let value: &[_; 4] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.u32_from_bytes(*value);
		visitor.visit_u32(value)
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 8)];
		self.pos += 8;

		let value: &[_; 8] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.u64_from_bytes(*value);
		visitor.visit_u64(value)
	}

	fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..(self.pos + 8)];
		self.pos += 8;

		let value: &[_; 8] = std::convert::TryInto::try_into(value).expect("infallible");
		let value = self.endianness.f64_from_bytes(*value);
		visitor.visit_f64(value)
	}

	fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		let len: u32 = serde::Deserialize::deserialize(&mut *self)?;
		let len: usize = std::convert::TryInto::try_into(len).map_err(DeserializeError::ExceedsNumericLimits)?;

		if self.buf.len() < self.pos + len + 1 {
			return Err(DeserializeError::EndOfInput);
		}
		if self.buf[self.pos + len] != b'\0' {
			return Err(DeserializeError::StringMissingNulTerminator);
		}

		let data = &self.buf[self.pos..(self.pos + len)];
		self.pos += len + 1;

		let s = std::str::from_utf8(data).map_err(DeserializeError::InvalidUtf8)?;
		visitor.visit_borrowed_str(s)
	}

	fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		let data_len: u32 = serde::de::Deserialize::deserialize(&mut *self)?;
		let data_len: usize = std::convert::TryInto::try_into(data_len).map_err(serde::de::Error::custom)?;

		let data_end_pos = self.pos + data_len;

		let result = visitor.visit_seq(SeqDeserializer { inner: self, data_end_pos })?;

		Ok(result)
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		visitor.visit_seq(TupleDeserializer(self))
	}

	fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		self.pad_to(8)?;

		visitor.visit_map(StructDeserializer(self))
	}

	fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		unimplemented!();
	}

	fn is_human_readable(&self) -> bool {
		false
	}
}

struct SeqDeserializer<'de, 'a> {
	inner: &'a mut Deserializer<'de>,
	data_end_pos: usize,
}

impl<'de, 'a> serde::de::SeqAccess<'de> for SeqDeserializer<'de, 'a> {
	type Error = DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: serde::de::DeserializeSeed<'de> {
		if self.inner.pos >= self.data_end_pos {
			// self.inner.pos will be greater than self.data_end_pos in case there was padding between the length and the first element,
			// because self.data_end_pos was calculated without considering that padding.
			Ok(None)
		}
		else {
			seed.deserialize(&mut *self.inner).map(Some)
		}
	}
}

struct TupleDeserializer<'de, 'a>(&'a mut Deserializer<'de>);

impl<'de, 'a> serde::de::SeqAccess<'de> for TupleDeserializer<'de, 'a> {
	type Error = DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: serde::de::DeserializeSeed<'de> {
		seed.deserialize(&mut *self.0).map(Some)
	}
}

struct StructDeserializer<'de, 'a>(&'a mut Deserializer<'de>);

impl<'de, 'a> serde::de::MapAccess<'de> for StructDeserializer<'de, 'a> {
	type Error = DeserializeError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> where K: serde::de::DeserializeSeed<'de> {
		seed.deserialize(&mut *self.0).map(Some)
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> where V: serde::de::DeserializeSeed<'de> {
		seed.deserialize(&mut *self.0)
	}
}

/// An error from deserializing a value using the D-Bus binary protocol.
#[derive(Debug)]
pub enum DeserializeError {
	ArrayElementDoesntMatchSignature { expected: crate::types::Signature, actual: crate::types::Signature },
	Custom(String),
	DeserializeAnyNotSupported,
	EndOfInput,
	ExceedsNumericLimits(std::num::TryFromIntError),
	InvalidUtf8(std::str::Utf8Error),
	NonZeroPadding,
	StringMissingNulTerminator,
	Unexpected(String),
}

impl std::fmt::Display for DeserializeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		#[allow(clippy::match_same_arms)]
		match self {
			DeserializeError::ArrayElementDoesntMatchSignature { expected, actual } => write!(f,
				"array has element signature {} but it contains an element with signature {}",
				expected, actual,
			),
			DeserializeError::Custom(message) => f.write_str(message),
			DeserializeError::DeserializeAnyNotSupported => f.write_str("deserialize_any is not supported"),
			DeserializeError::EndOfInput => f.write_str("end of input"),
			DeserializeError::ExceedsNumericLimits(_) => f.write_str("value exceeds numeric limits"),
			DeserializeError::InvalidUtf8(_) => f.write_str("deserialized string is not valid UTF-8"),
			DeserializeError::NonZeroPadding => f.write_str("padding contains a byte other than 0x00"),
			DeserializeError::StringMissingNulTerminator => f.write_str("deserialized string is not nul-terminated"),
			DeserializeError::Unexpected(message) => f.write_str(message),
		}
	}
}

impl std::error::Error for DeserializeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		#[allow(clippy::match_same_arms)]
		match self {
			DeserializeError::ArrayElementDoesntMatchSignature { expected: _, actual: _ } => None,
			DeserializeError::Custom(_) => None,
			DeserializeError::DeserializeAnyNotSupported => None,
			DeserializeError::EndOfInput => None,
			DeserializeError::ExceedsNumericLimits(err) => Some(err),
			DeserializeError::InvalidUtf8(err) => Some(err),
			DeserializeError::NonZeroPadding => None,
			DeserializeError::StringMissingNulTerminator => None,
			DeserializeError::Unexpected(_) => None,
		}
	}
}

impl serde::de::Error for DeserializeError {
	fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
		DeserializeError::Custom(msg.to_string())
	}
}
