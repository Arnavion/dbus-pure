#[derive(Debug)]
pub(crate) struct Serializer<'ser> {
	buf: &'ser mut Vec<u8>,
	start: usize,

	// Used by SeqSerializer to know how much padding was inserted before the first element of the array.
	// Each serializer appends a new value into the Vec when it's created, and pops it out when it's done.
	// Serializer itself updates the last value in the Vec whenever its pad_to is called if the last value is None.
	array_start_paddings: Vec<Option<usize>>,
}

impl<'ser> Serializer<'ser> {
	pub(crate) fn new(buf: &'ser mut Vec<u8>) -> Self {
		let start = buf.len();
		Serializer {
			buf,
			start,
			array_start_paddings: vec![],
		}
	}

	pub(crate) fn pad_to(&mut self, alignment: usize) {
		let pos = self.buf.len() - self.start;
		let new_pos = ((pos + alignment - 1) / alignment) * alignment;
		let new_len = self.start + new_pos;
		self.buf.resize(new_len, 0);

		if let Some(last_array_start_paddings @ None) = self.array_start_paddings.last_mut() {
			*last_array_start_paddings = Some(new_pos - pos);
		}
	}
}

impl<'ser, 'a> serde::Serializer for &'a mut Serializer<'ser> {
	type Ok = ();
	type Error = SerializeError;
	type SerializeSeq = SeqSerializer<'ser, 'a>;
	type SerializeTuple = TupleSerializer<'ser, 'a>;
	type SerializeTupleStruct = TupleStructSerializer;
	type SerializeTupleVariant = TupleVariantSerializer;
	type SerializeMap = MapSerializer;
	type SerializeStruct = StructSerializer<'ser, 'a>;
	type SerializeStructVariant = StructVariantSerializer;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.pad_to(4);
		let v: u32 = if v { 1 } else { 0 };
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.pad_to(2);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.pad_to(4);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		self.pad_to(8);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.buf.push(v);
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.pad_to(2);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.pad_to(4);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.pad_to(8);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		self.pad_to(8);
		self.buf.extend_from_slice(&v.to_le_bytes());
		Ok(())
	}

	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		serde::Serialize::serialize(v.as_bytes(), &mut *self)?;
		serde::Serialize::serialize(&b'\0', &mut *self)?;
		Ok(())
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}

	fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		// Insert a record for the new serializer before serializing the length,
		// so that serializing the 0 length below does not modify the previous serializer's value (if any).
		self.array_start_paddings.push(None);

		serde::Serialize::serialize(&0_u32, &mut *self)?;
		let data_len_pos = self.buf.len() - 4;

		// ... and reset the padding back to None to forget about the u32 len's padding.
		*self.array_start_paddings.last_mut().unwrap() = None;

		Ok(SeqSerializer {
			inner: self,
			data_len_pos,
		})
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Ok(TupleSerializer(self))
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		unimplemented!();
	}

	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		unimplemented!();
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		unimplemented!();
	}

	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		self.pad_to(8);
		Ok(StructSerializer(self))
	}

	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		unimplemented!();
	}

	fn is_human_readable(&self) -> bool {
		false
	}
}

pub(crate) struct SeqSerializer<'ser, 'a> {
	inner: &'a mut Serializer<'ser>,
	data_len_pos: usize,
}

impl<'ser, 'a> serde::ser::SerializeSeq for SeqSerializer<'ser, 'a> {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		value.serialize(&mut *self.inner)?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let last_array_start_padding =
			self.inner.array_start_paddings.pop()
			.unwrap()
			.unwrap_or(0);
		let data_start_pos = self.data_len_pos + 4 + last_array_start_padding;

		let data_end_pos = self.inner.buf.len();

		let data_len: u32 = std::convert::TryInto::try_into(data_end_pos - data_start_pos).map_err(serde::ser::Error::custom)?;

		self.inner.buf[self.data_len_pos..(self.data_len_pos + 4)].copy_from_slice(&data_len.to_le_bytes());

		Ok(())
	}
}

pub(crate) struct TupleSerializer<'ser, 'a>(&'a mut Serializer<'ser>);

impl<'ser, 'a> serde::ser::SerializeTuple for TupleSerializer<'ser, 'a> {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		value.serialize(&mut *self.0)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

pub(crate) struct TupleStructSerializer;

impl serde::ser::SerializeTupleStruct for TupleStructSerializer {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}
}

pub(crate) struct TupleVariantSerializer;

impl serde::ser::SerializeTupleVariant for TupleVariantSerializer {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}
}

pub(crate) struct MapSerializer;

impl serde::ser::SerializeMap for MapSerializer {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}
}

pub(crate) struct StructSerializer<'ser, 'a>(&'a mut Serializer<'ser>);

impl<'ser, 'a> serde::ser::SerializeStruct for StructSerializer<'ser, 'a> {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		value.serialize(&mut *self.0)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

pub(crate) struct StructVariantSerializer;

impl serde::ser::SerializeStructVariant for StructVariantSerializer {
	type Ok = ();
	type Error = SerializeError;

	fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> where T: serde::Serialize + ?Sized {
		unimplemented!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unimplemented!();
	}
}

/// An error from serializing a value using the D-Bus binary protocol.
#[derive(Debug)]
pub enum SerializeError {
	Custom(String),
	ExceedsNumericLimits(std::num::TryFromIntError),
	Write(std::io::Error),
}

impl std::fmt::Display for SerializeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SerializeError::Custom(message) => f.write_str(message),
			SerializeError::ExceedsNumericLimits(_) => f.write_str("value exceeds numeric limits"),
			SerializeError::Write(_) => f.write_str("could not write message"),
		}
	}
}

impl std::error::Error for SerializeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			SerializeError::Custom(_) => None,
			SerializeError::ExceedsNumericLimits(err) => Some(err),
			SerializeError::Write(err) => Some(err),
		}
	}
}

impl serde::ser::Error for SerializeError {
	fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
		SerializeError::Custom(msg.to_string())
	}
}
