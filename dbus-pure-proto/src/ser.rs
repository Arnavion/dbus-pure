#[derive(Debug)]
pub(crate) struct Serializer<'ser> {
	buf: &'ser mut Vec<u8>,
	start: usize,
	endianness: crate::Endianness,
}

impl<'ser> Serializer<'ser> {
	pub(crate) fn new(buf: &'ser mut Vec<u8>, endianness: crate::Endianness) -> Self {
		let start = buf.len();
		Serializer {
			endianness,
			buf,
			start,
		}
	}

	pub(crate) fn pad_to(&mut self, alignment: usize) {
		let pos = self.buf.len() - self.start;
		let new_pos = ((pos + alignment - 1) / alignment) * alignment;
		let new_len = self.start + new_pos;
		self.buf.resize(new_len, 0);
	}

	pub(crate) fn serialize_array<T>(
		&mut self,
		element_alignment: usize,
		v: &[T],
		mut f: impl FnMut(&T, &mut Self) -> Result<(), SerializeError>,
	) -> Result<(), SerializeError> {
		self.serialize_u32(0)?;
		let data_len_pos = self.buf.len() - 4;

		self.pad_to(element_alignment);

		let data_start_pos = self.buf.len();

		for v in v {
			f(v, self)?;
		}

		let data_end_pos = self.buf.len();

		let data_len: u32 = std::convert::TryInto::try_into(data_end_pos - data_start_pos).map_err(crate::SerializeError::ExceedsNumericLimits)?;

		self.buf[data_len_pos..(data_len_pos + 4)].copy_from_slice(&self.endianness.u32_to_bytes(data_len));

		Ok(())
	}

	pub(crate) fn serialize_array_u8(
		&mut self,
		v: &[u8],
	) -> Result<(), SerializeError> {
		let data_len: u32 = std::convert::TryInto::try_into(v.len()).map_err(crate::SerializeError::ExceedsNumericLimits)?;
		self.serialize_u32(data_len)?;

		self.buf.extend_from_slice(v);

		Ok(())
	}

	pub(crate) fn serialize_bool(&mut self, v: bool) -> Result<(), SerializeError> {
		let v: u32 = if v { 1 } else { 0 };
		self.serialize_u32(v)
	}

	pub(crate) fn serialize_f64(&mut self, v: f64) -> Result<(), SerializeError> {
		self.pad_to(8);
		self.buf.extend_from_slice(&self.endianness.f64_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_i16(&mut self, v: i16) -> Result<(), SerializeError> {
		self.pad_to(2);
		self.buf.extend_from_slice(&self.endianness.i16_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_i32(&mut self, v: i32) -> Result<(), SerializeError> {
		self.pad_to(4);
		self.buf.extend_from_slice(&self.endianness.i32_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_i64(&mut self, v: i64) -> Result<(), SerializeError> {
		self.pad_to(8);
		self.buf.extend_from_slice(&self.endianness.i64_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_string(&mut self, v: &str) -> Result<(), SerializeError> {
		self.serialize_array(
			1,
			v.as_bytes(),
			|v, serializer| serializer.serialize_u8(*v),
		)?;
		self.serialize_u8(b'\0')?;
		Ok(())
	}

	pub(crate) fn serialize_struct(
		&mut self,
		f: impl FnOnce(&mut Self) -> Result<(), SerializeError>,
	) -> Result<(), SerializeError> {
		self.pad_to(8);

		f(self)?;
		Ok(())
	}

	pub(crate) fn serialize_u8(&mut self, v: u8) -> Result<(), SerializeError> {
		self.buf.push(v);
		Ok(())
	}

	pub(crate) fn serialize_u16(&mut self, v: u16) -> Result<(), SerializeError> {
		self.pad_to(2);
		self.buf.extend_from_slice(&self.endianness.u16_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_u32(&mut self, v: u32) -> Result<(), SerializeError> {
		self.pad_to(4);
		self.buf.extend_from_slice(&self.endianness.u32_to_bytes(v));
		Ok(())
	}

	pub(crate) fn serialize_u64(&mut self, v: u64) -> Result<(), SerializeError> {
		self.pad_to(8);
		self.buf.extend_from_slice(&self.endianness.u64_to_bytes(v));
		Ok(())
	}
}

/// An error from serializing a value using the D-Bus binary protocol.
#[derive(Debug)]
pub enum SerializeError {
	ExceedsNumericLimits(std::num::TryFromIntError),
}

impl std::fmt::Display for SerializeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SerializeError::ExceedsNumericLimits(_) => f.write_str("value exceeds numeric limits"),
		}
	}
}

impl std::error::Error for SerializeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			SerializeError::ExceedsNumericLimits(err) => Some(err),
		}
	}
}
