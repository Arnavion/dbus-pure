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
				return Err(DeserializeError::NonZeroPadding { start: self.pos, end: new_pos });
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

	pub(crate) fn deserialize_array<T>(
		&mut self,
		element_alignment: usize,
		mut f: impl FnMut(&mut Deserializer<'de>) -> Result<T, DeserializeError>,
	) -> Result<Vec<T>, DeserializeError> {
		let data_len = self.deserialize_u32()?;
		let data_len: usize = data_len.try_into().map_err(crate::DeserializeError::ExceedsNumericLimits)?;

		self.pad_to(element_alignment)?;

		let data_end_pos = self.pos + data_len;

		let mut inner = Deserializer {
			buf: self.buf.get(..data_end_pos).ok_or(DeserializeError::EndOfInput)?,
			pos: self.pos,
			endianness: self.endianness,
		};

		let mut result = vec![];

		while inner.pos != inner.buf.len() {
			result.push(f(&mut inner)?);
		}

		self.pos = inner.pos;

		Ok(result)
	}

	pub(crate) fn deserialize_array_u8(&mut self) -> Result<&'de [u8], DeserializeError> {
		let data_len = self.deserialize_u32()?;
		let data_len: usize = data_len.try_into().map_err(crate::DeserializeError::ExceedsNumericLimits)?;

		let data_end_pos = self.pos + data_len;

		let result = self.buf.get(self.pos..data_end_pos).ok_or(DeserializeError::EndOfInput)?;

		self.pos = data_end_pos;

		Ok(result)
	}

	pub(crate) fn deserialize_bool(&mut self) -> Result<bool, DeserializeError> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..4];
		self.pos += 4;

		let value: &[_; 4] = value.try_into().expect("infallible");
		let value = self.endianness.u32_from_bytes(*value);
		match value {
			0 => Ok(false),
			1 => Ok(true),
			value => Err(DeserializeError::InvalidValue { expected: "0 or 1".into(), actual: value.to_string() }),
		}
	}

	pub(crate) fn deserialize_f64(&mut self) -> Result<f64, DeserializeError> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..8];
		self.pos += 8;

		let value: &[_; 8] = value.try_into().expect("infallible");
		let value = self.endianness.f64_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_i16(&mut self) -> Result<i16, DeserializeError> {
		self.pad_to(2)?;

		if self.buf.len() < self.pos + 2 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..2];
		self.pos += 2;

		let value: &[_; 2] = value.try_into().expect("infallible");
		let value = self.endianness.i16_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_i32(&mut self) -> Result<i32, DeserializeError> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..4];
		self.pos += 4;

		let value: &[_; 4] = value.try_into().expect("infallible");
		let value = self.endianness.i32_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_i64(&mut self) -> Result<i64, DeserializeError> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..8];
		self.pos += 8;

		let value: &[_; 8] = value.try_into().expect("infallible");
		let value = self.endianness.i64_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_string(&mut self) -> Result<&'de str, DeserializeError> {
		let data = self.deserialize_array_u8()?;

		let nul = self.deserialize_u8()?;
		if nul != b'\0' {
			return Err(DeserializeError::StringMissingNulTerminator);
		}

		let s = std::str::from_utf8(data).map_err(DeserializeError::InvalidUtf8)?;
		Ok(s)
	}

	pub(crate) fn deserialize_struct<T>(
		&mut self,
		f: impl FnOnce(&mut Deserializer<'de>) -> Result<T, DeserializeError>,
	) -> Result<T, DeserializeError> {
		self.pad_to(8)?;

		f(self)
	}

	pub(crate) fn deserialize_u8(&mut self) -> Result<u8, DeserializeError> {
		if self.buf.len() < self.pos + 1 {
			return Err(DeserializeError::EndOfInput);
		}

		let value = self.buf[self.pos];
		self.pos += 1;

		Ok(value)
	}

	pub(crate) fn deserialize_u16(&mut self) -> Result<u16, DeserializeError> {
		self.pad_to(2)?;

		if self.buf.len() < self.pos + 2 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..2];
		self.pos += 2;

		let value: &[_; 2] = value.try_into().expect("infallible");
		let value = self.endianness.u16_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_u32(&mut self) -> Result<u32, DeserializeError> {
		self.pad_to(4)?;

		if self.buf.len() < self.pos + 4 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..4];
		self.pos += 4;

		let value: &[_; 4] = value.try_into().expect("infallible");
		let value = self.endianness.u32_from_bytes(*value);
		Ok(value)
	}

	pub(crate) fn deserialize_u64(&mut self) -> Result<u64, DeserializeError> {
		self.pad_to(8)?;

		if self.buf.len() < self.pos + 8 {
			return Err(DeserializeError::EndOfInput);
		}

		let value: &[_] = &self.buf[self.pos..][..8];
		self.pos += 8;

		let value: &[_; 8] = value.try_into().expect("infallible");
		let value = self.endianness.u64_from_bytes(*value);
		Ok(value)
	}
}

/// An error from deserializing a value using the D-Bus binary protocol.
#[derive(Debug)]
pub enum DeserializeError {
	EndOfInput,
	ExceedsNumericLimits(std::num::TryFromIntError),
	InvalidUtf8(std::str::Utf8Error),
	InvalidValue { expected: std::borrow::Cow<'static, str>, actual: String },
	MissingRequiredMessageHeaderField { method_name: &'static str, header_field_name: &'static str },
	NonZeroPadding { start: usize, end: usize },
	StringMissingNulTerminator,
}

impl std::fmt::Display for DeserializeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		#[allow(clippy::match_same_arms)]
		match self {
			DeserializeError::EndOfInput => f.write_str("end of input"),
			DeserializeError::ExceedsNumericLimits(_) => f.write_str("value exceeds numeric limits"),
			DeserializeError::InvalidUtf8(_) => f.write_str("deserialized string is not valid UTF-8"),
			DeserializeError::InvalidValue { expected, actual } => write!(f, "expected {expected} but got {actual}"),
			DeserializeError::MissingRequiredMessageHeaderField { method_name, header_field_name } =>
				write!(f, "{method_name} message is missing {header_field_name} required header field"),
			DeserializeError::NonZeroPadding { start, end } => write!(f, "padding contains a byte other than 0x00 between positions {start} and {end}"),
			DeserializeError::StringMissingNulTerminator => f.write_str("deserialized string is not nul-terminated"),
		}
	}
}

impl std::error::Error for DeserializeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		#[allow(clippy::match_same_arms)]
		match self {
			DeserializeError::EndOfInput => None,
			DeserializeError::ExceedsNumericLimits(err) => Some(err),
			DeserializeError::InvalidUtf8(err) => Some(err),
			DeserializeError::InvalidValue { expected: _, actual: _ } => None,
			DeserializeError::MissingRequiredMessageHeaderField { method_name: _, header_field_name: _ } => None,
			DeserializeError::NonZeroPadding { start: _, end: _ } => None,
			DeserializeError::StringMissingNulTerminator => None,
		}
	}
}
