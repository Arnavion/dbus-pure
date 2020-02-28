/// A variant. It can store any kind of data type that D-Bus supports.
#[derive(Clone, Debug, PartialEq)]
pub enum Variant<'a> {
	/// An array of variants. All variants must have the same signature as `element_signature`.
	///
	/// Simpler arrays will be deserialized as the other `Array*` variants.
	/// For example, byte arrays (`ay`) will always be deserialized as `ArrayU8`.
	Array {
		element_signature: crate::Signature,
		elements: crate::std2::CowSlice<'a, Variant<'a>>,
	},

	/// Simpler wrapper over a bool array (`ab`) than the generic `Array` variant.
	ArrayBool(std::borrow::Cow<'a, [bool]>),

	/// Simpler wrapper over a double array (`ad`) than the generic `Array` variant.
	ArrayF64(std::borrow::Cow<'a, [f64]>),

	/// Simpler wrapper over an i16 array (`an`) than the generic `Array` variant.
	ArrayI16(std::borrow::Cow<'a, [i16]>),

	/// Simpler wrapper over an i32 array (`ai`) than the generic `Array` variant.
	ArrayI32(std::borrow::Cow<'a, [i32]>),

	/// Simpler wrapper over an i64 array (`ax`) than the generic `Array` variant.
	ArrayI64(std::borrow::Cow<'a, [i64]>),

	/// Simpler wrapper over a string array (`as`) than the generic `Array` variant.
	ArrayString(std::borrow::Cow<'a, [std::borrow::Cow<'a, str>]>),

	/// Simpler wrapper over a u8 array (`ay`) than the generic `Array` variant.
	ArrayU8(std::borrow::Cow<'a, [u8]>),

	/// Simpler wrapper over a u16 array (`aq`) than the generic `Array` variant.
	ArrayU16(std::borrow::Cow<'a, [u16]>),

	/// Simpler wrapper over a u32 array (`au`) than the generic `Array` variant.
	ArrayU32(std::borrow::Cow<'a, [u32]>),

	/// Simpler wrapper over a u64 array (`at`) than the generic `Array` variant.
	ArrayU64(std::borrow::Cow<'a, [u64]>),

	ArrayUnixFd(std::borrow::Cow<'a, [crate::UnixFd]>),

	Bool(bool),

	DictEntry {
		key: crate::std2::CowRef<'a, Variant<'a>>,
		value: crate::std2::CowRef<'a, Variant<'a>>,
	},

	F64(f64),

	I16(i16),

	I32(i32),

	I64(i64),

	ObjectPath(crate::ObjectPath<'a>),

	Signature(crate::Signature),

	String(std::borrow::Cow<'a, str>),

	Struct {
		fields: crate::std2::CowSlice<'a, Variant<'a>>,
	},

	/// A sequence of signatures.
	///
	/// A message body with one or more parameters is of this type. For example, if a method takes two parameters of type string and byte,
	/// the body should be a `Variant::Tuple { elements: (&[Variant::String(...), Variant::U8(...)][..]).into() }`
	Tuple {
		elements: crate::std2::CowSlice<'a, Variant<'a>>,
	},

	U8(u8),

	U16(u16),

	U32(u32),

	U64(u64),

	UnixFd(crate::UnixFd),

	Variant(crate::std2::CowRef<'a, Variant<'a>>),
}

impl<'a> Variant<'a> {
	/// Convenience function to view this `Variant` as a `&[Variant]` if it's an array and its elements have the given signature.
	pub fn as_array<'b>(&'b self, expected_element_signature: &crate::Signature) -> Option<&'b [Variant<'a>]> {
		match self {
			Variant::Array { element_signature, elements } if element_signature == expected_element_signature => Some(elements),
			_ => None,
		}
	}

	/// Convenience function to view this `Variant` as a `&[Cow<'_, str>]` if it's an array of strings.
	pub fn as_array_string<'b>(&'b self) -> Option<&'b [std::borrow::Cow<'a, str>]> {
		match self {
			Variant::ArrayString(elements) => Some(elements),
			_ => None,
		}
	}

	/// Convenience function to view this `Variant` as a `bool` if it is one.
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Variant::Bool(value) => Some(*value),
			_ => None,
		}
	}

	/// Convenience function to view this `Variant` as a `&str` if it's a string.
	pub fn as_string(&self) -> Option<&str> {
		match self {
			Variant::String(value) => Some(value),
			_ => None,
		}
	}

	/// Convenience function to view this `Variant` as a `u32` if it is one.
	pub fn as_u32(&self) -> Option<u32> {
		match self {
			Variant::U32(value) => Some(*value),
			_ => None,
		}
	}

	/// Convenience function to view this `Variant` as its inner `Variant` if it has one.
	pub fn as_variant<'b>(&'b self) -> Option<&'b Variant<'a>> {
		match self {
			Variant::Variant(value) => Some(value),
			_ => None,
		}
	}

	pub(crate) fn inner_signature(&self) -> crate::Signature {
		match self {
			Variant::Array { element_signature, elements: _ } =>
				crate::Signature::Array { element: Box::new(element_signature.clone()) },

			Variant::ArrayBool(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::Bool) },

			Variant::ArrayF64(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::F64) },

			Variant::ArrayI16(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::I16) },

			Variant::ArrayI32(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::I32) },

			Variant::ArrayI64(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::I64) },

			Variant::ArrayString(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::String) },

			Variant::ArrayU8(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::U8) },

			Variant::ArrayU16(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::U16) },

			Variant::ArrayU32(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::U32) },

			Variant::ArrayU64(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::U64) },

			Variant::ArrayUnixFd(_) =>
				crate::Signature::Array { element: Box::new(crate::Signature::UnixFd) },

			Variant::Bool(_) =>
				crate::Signature::Bool,

			Variant::DictEntry { key, value } =>
				crate::Signature::DictEntry {
					key: Box::new(key.inner_signature()),
					value: Box::new(value.inner_signature()),
				},

			Variant::F64(_) =>
				crate::Signature::F64,

			Variant::I16(_) =>
				crate::Signature::I16,

			Variant::I32(_) =>
				crate::Signature::I32,

			Variant::I64(_) =>
				crate::Signature::I64,

			Variant::ObjectPath(_) =>
				crate::Signature::ObjectPath,

			Variant::Signature(_) =>
				crate::Signature::Signature,

			Variant::String(_) =>
				crate::Signature::String,

			Variant::Struct { fields } =>
				crate::Signature::Struct { fields: fields.iter().map(Variant::inner_signature).collect() },

			Variant::Tuple { elements } =>
				crate::Signature::Tuple { elements: elements.iter().map(Variant::inner_signature).collect() },

			Variant::U8(_) =>
				crate::Signature::U8,

			Variant::U16(_) =>
				crate::Signature::U16,

			Variant::U32(_) =>
				crate::Signature::U32,

			Variant::U64(_) =>
				crate::Signature::U64,

			Variant::UnixFd(_) =>
				crate::Signature::UnixFd,

			Variant::Variant(_) =>
				crate::Signature::Variant,
		}
	}
}

impl<'de> Variant<'de> {
	pub(crate) fn deserialize(deserializer: &mut crate::de::Deserializer<'de>, signature: &crate::Signature) -> Result<Self, crate::DeserializeError> {
		match signature {
			crate::Signature::Array { element } => match &**element {
				crate::Signature::Bool => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_bool)?;
					Ok(Variant::ArrayBool(elements.into()))
				},

				crate::Signature::F64 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_f64)?;
					Ok(Variant::ArrayF64(elements.into()))
				},

				crate::Signature::I16 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_i16)?;
					Ok(Variant::ArrayI16(elements.into()))
				},

				crate::Signature::I32 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_i32)?;
					Ok(Variant::ArrayI32(elements.into()))
				},

				crate::Signature::I64 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_i64)?;
					Ok(Variant::ArrayI64(elements.into()))
				},

				crate::Signature::String => {
					let elements = deserializer.deserialize_array(element.alignment(), |deserializer| Ok(deserializer.deserialize_string()?.into()))?;
					Ok(Variant::ArrayString(elements.into()))
				},

				crate::Signature::U8 => {
					let elements = deserializer.deserialize_array_u8()?;
					Ok(Variant::ArrayU8(elements.into()))
				},

				crate::Signature::U16 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_u16)?;
					Ok(Variant::ArrayU16(elements.into()))
				},

				crate::Signature::U32 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_u32)?;
					Ok(Variant::ArrayU32(elements.into()))
				},

				crate::Signature::U64 => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::de::Deserializer::deserialize_u64)?;
					Ok(Variant::ArrayU64(elements.into()))
				},

				crate::Signature::UnixFd => {
					let elements = deserializer.deserialize_array(element.alignment(), crate::UnixFd::deserialize)?;
					Ok(Variant::ArrayUnixFd(elements.into()))
				},

				element_signature => {
					let elements = deserializer.deserialize_array(element.alignment(), |deserializer| Self::deserialize(deserializer, element))?;
					Ok(Variant::Array { element_signature: element_signature.clone(), elements: elements.into() })
				},
			},

			crate::Signature::Bool => {
				let value = deserializer.deserialize_bool()?;
				Ok(Variant::Bool(value))
			},

			crate::Signature::DictEntry { key, value } => {
				let (key, value) = deserializer.deserialize_struct(|deserializer| {
					let key = Self::deserialize(deserializer, key)?;
					let value = Self::deserialize(deserializer, value)?;
					Ok((key, value))
				})?;
				Ok(Variant::DictEntry { key: Box::new(key).into(), value: Box::new(value).into() })
			},

			crate::Signature::F64 => {
				let value = deserializer.deserialize_f64()?;
				Ok(Variant::F64(value))
			},

			crate::Signature::I16 => {
				let value = deserializer.deserialize_i16()?;
				Ok(Variant::I16(value))
			},

			crate::Signature::I32 => {
				let value = deserializer.deserialize_i32()?;
				Ok(Variant::I32(value))
			},

			crate::Signature::I64 => {
				let value = deserializer.deserialize_i64()?;
				Ok(Variant::I64(value))
			},

			crate::Signature::ObjectPath => {
				let value = crate::ObjectPath::deserialize(deserializer)?;
				Ok(Variant::ObjectPath(value))
			},

			crate::Signature::Signature => {
				let value = crate::Signature::deserialize(deserializer)?;
				Ok(Variant::Signature(value))
			},

			crate::Signature::String => {
				let value = deserializer.deserialize_string()?.into();
				Ok(Variant::String(value))
			},

			crate::Signature::Struct { fields } => {
				let fields = deserializer.deserialize_struct(|deserializer| {
					let fields: Result<Vec<_>, _> =
						fields.iter()
						.map(|field| {
							let field = Self::deserialize(deserializer, field)?;
							Ok(field)
						})
						.collect();
					let fields = fields?;
					Ok(fields)
				})?;
				Ok(Variant::Struct { fields: fields.into() })
			},

			crate::Signature::Tuple { elements } => {
				let elements: Result<Vec<_>, _> =
					elements.iter()
					.map(|element| {
						let element = Self::deserialize(deserializer, element)?;
						Ok(element)
					})
					.collect();
				let elements = elements?;
				Ok(Variant::Tuple { elements: elements.into() })
			},

			crate::Signature::U8 => {
				let value = deserializer.deserialize_u8()?;
				Ok(Variant::U8(value))
			},

			crate::Signature::U16 => {
				let value = deserializer.deserialize_u16()?;
				Ok(Variant::U16(value))
			},

			crate::Signature::U32 => {
				let value = deserializer.deserialize_u32()?;
				Ok(Variant::U32(value))
			},

			crate::Signature::U64 => {
				let value = deserializer.deserialize_u64()?;
				Ok(Variant::U64(value))
			},

			crate::Signature::UnixFd => {
				let value = crate::UnixFd::deserialize(deserializer)?;
				Ok(Variant::UnixFd(value))
			},

			crate::Signature::Variant => {
				let signature = crate::Signature::deserialize(deserializer)?;
				let value = Self::deserialize(deserializer, &signature)?;
				Ok(Variant::Variant(Box::new(value).into()))
			},
		}
	}

	pub fn into_owned(self) -> Variant<'static> {
		match self {
			Variant::Array { element_signature, elements } => Variant::Array {
				element_signature,
				elements: elements.into_owned().into_iter().map(Self::into_owned).collect::<Vec<_>>().into(),
			},

			Variant::ArrayBool(elements) =>
				Variant::ArrayBool(elements.into_owned().into()),

			Variant::ArrayF64(elements) =>
				Variant::ArrayF64(elements.into_owned().into()),

			Variant::ArrayI16(elements) =>
				Variant::ArrayI16(elements.into_owned().into()),

			Variant::ArrayI32(elements) =>
				Variant::ArrayI32(elements.into_owned().into()),

			Variant::ArrayI64(elements) =>
				Variant::ArrayI64(elements.into_owned().into()),

			Variant::ArrayString(elements) => Variant::ArrayString(
				elements.into_owned()
				.into_iter()
				.map(|element| element.into_owned().into())
				.collect::<Vec<_>>()
				.into(),
			),

			Variant::ArrayU8(elements) =>
				Variant::ArrayU8(elements.into_owned().into()),

			Variant::ArrayU16(elements) =>
				Variant::ArrayU16(elements.into_owned().into()),

			Variant::ArrayU32(elements) =>
				Variant::ArrayU32(elements.into_owned().into()),

			Variant::ArrayU64(elements) =>
				Variant::ArrayU64(elements.into_owned().into()),

			Variant::ArrayUnixFd(elements) =>
				Variant::ArrayUnixFd(elements.into_owned().into()),

			Variant::Bool(value) =>
				Variant::Bool(value),

			Variant::DictEntry { key, value } => Variant::DictEntry {
				key: Box::new(key.into_owned().into_owned()).into(),
				value: Box::new(value.into_owned().into_owned()).into(),
			},

			Variant::F64(value) =>
				Variant::F64(value),

			Variant::I16(value) =>
				Variant::I16(value),

			Variant::I32(value) =>
				Variant::I32(value),

			Variant::I64(value) =>
				Variant::I64(value),

			Variant::ObjectPath(value) =>
				Variant::ObjectPath(value.into_owned()),

			Variant::Signature(value) =>
				Variant::Signature(value),

			Variant::String(value) =>
				Variant::String(value.into_owned().into()),

			Variant::Struct { fields } => Variant::Struct {
				fields:
					fields.into_owned()
					.into_iter()
					.map(Self::into_owned)
					.collect::<Vec<_>>()
					.into(),
			},

			Variant::Tuple { elements } => Variant::Tuple {
				elements:
					elements.into_owned()
					.into_iter()
					.map(Self::into_owned)
					.collect::<Vec<_>>()
					.into(),
			},

			Variant::U8(value) =>
				Variant::U8(value),

			Variant::U16(value) =>
				Variant::U16(value),

			Variant::U32(value) =>
				Variant::U32(value),

			Variant::U64(value) =>
				Variant::U64(value),

			Variant::UnixFd(value) =>
				Variant::UnixFd(value),

			Variant::Variant(value) =>
				Variant::Variant(Box::new(value.into_owned().into_owned()).into()),
		}
	}
}

impl Variant<'_> {
	pub(crate) fn serialize(&self, serializer: &mut crate::ser::Serializer<'_>) -> Result<(), crate::SerializeError> {
		match self {
			Variant::Array { element_signature, elements } =>
				serializer.serialize_array(
					element_signature.alignment(),
					elements,
					Self::serialize,
				),

			Variant::ArrayBool(elements) =>
				serializer.serialize_array(
					4,
					elements,
					|v, serializer| serializer.serialize_bool(*v),
				),

			Variant::ArrayF64(elements) =>
				serializer.serialize_array(
					8,
					elements,
					|v, serializer| serializer.serialize_f64(*v),
				),

			Variant::ArrayI16(elements) =>
				serializer.serialize_array(
					2,
					elements,
					|v, serializer| serializer.serialize_i16(*v),
				),

			Variant::ArrayI32(elements) =>
				serializer.serialize_array(
					4,
					elements,
					|v, serializer| serializer.serialize_i32(*v),
				),

			Variant::ArrayI64(elements) =>
				serializer.serialize_array(
					8,
					elements,
					|v, serializer| serializer.serialize_i64(*v),
				),

			Variant::ArrayString(elements) =>
				serializer.serialize_array(
					4,
					elements,
					|v, serializer| serializer.serialize_string(v),
				),

			Variant::ArrayU8(elements) =>
				serializer.serialize_array_u8(elements),

			Variant::ArrayU16(elements) =>
				serializer.serialize_array(
					2,
					elements,
					|v, serializer| serializer.serialize_u16(*v),
				),

			Variant::ArrayU32(elements) =>
				serializer.serialize_array(
					4,
					elements,
					|v, serializer| serializer.serialize_u32(*v),
				),

			Variant::ArrayU64(elements) =>
				serializer.serialize_array(
					8,
					elements,
					|v, serializer| serializer.serialize_u64(*v),
				),

			Variant::ArrayUnixFd(elements) =>
				serializer.serialize_array(
					4,
					elements,
					|v, serializer| v.serialize(serializer),
				),

			Variant::Bool(value) =>
				serializer.serialize_bool(*value),

			Variant::DictEntry { key, value } =>
				serializer.serialize_struct(|serializer| {
					key.serialize(serializer)?;
					value.serialize(serializer)?;
					Ok(())
				}),

			Variant::F64(value) =>
				serializer.serialize_f64(*value),

			Variant::I16(value) =>
				serializer.serialize_i16(*value),

			Variant::I32(value) =>
				serializer.serialize_i32(*value),

			Variant::I64(value) =>
				serializer.serialize_i64(*value),

			Variant::ObjectPath(value) =>
				value.serialize(serializer),

			Variant::Signature(value) =>
				value.serialize(serializer),

			Variant::String(value) =>
				serializer.serialize_string(value),

			Variant::Struct { fields } =>
				serializer.serialize_struct(|serializer| {
					for field in &**fields {
						field.serialize(serializer)?;
					}

					Ok(())
				}),

			Variant::Tuple { elements } => {
				for element in &**elements {
					element.serialize(serializer)?;
				}

				Ok(())
			},

			Variant::U8(value) =>
				serializer.serialize_u8(*value),

			Variant::U16(value) =>
				serializer.serialize_u16(*value),

			Variant::U32(value) =>
				serializer.serialize_u32(*value),

			Variant::U64(value) =>
				serializer.serialize_u64(*value),

			Variant::UnixFd(value) =>
				value.serialize(serializer),

			Variant::Variant(value) => {
				let signature = value.inner_signature();
				signature.serialize(serializer)?;
				value.serialize(serializer)?;
				Ok(())
			},
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_variant_serde() {
		fn test<'a>(
			signature: &str,
			expected_serialized: &'a [u8],
			expected_variant: super::Variant<'a>,
		) {
			let signature: crate::Signature = signature.parse().unwrap();

			let mut deserializer = crate::de::Deserializer::new(expected_serialized, 0, crate::Endianness::Little);
			let actual_variant = super::Variant::deserialize(&mut deserializer, &signature).unwrap();
			assert_eq!(expected_variant, actual_variant);

			assert_eq!(deserializer.pos(), expected_serialized.len());

			let mut actual_serialized = vec![];
			let mut serializer = crate::ser::Serializer::new(&mut actual_serialized, crate::Endianness::Little);
			actual_variant.serialize(&mut serializer).unwrap();
			assert_eq!(expected_serialized, &*actual_serialized);
		}

		test(
			"at",
			b"\
				\x08\x00\x00\x00\
				\x00\x00\x00\x00\
				\x08\x07\x06\x05\
				\x04\x03\x02\x01\
			",
			super::Variant::ArrayU64((&[
				0x01020304_05060708_u64,
			][..]).into()),
		);

		test(
			"yat",
			b"\
				\x05\
				\x00\x00\x00\
				\x08\x00\x00\x00\
				\x08\x07\x06\x05\
				\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::ArrayU64((&[
						0x01020304_05060708_u64,
					][..]).into()),
				][..]).into(),
			},
		);

		test(
			"at",
			b"\
				\x00\x00\x00\x00\
				\x00\x00\x00\x00\
			",
			super::Variant::ArrayU64((&[][..]).into()),
		);

		test(
			"yat",
			b"\
				\x05\
				\x00\x00\x00\
				\x00\x00\x00\x00\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::ArrayU64((&[][..]).into()),
				][..]).into(),
			},
		);

		test(
			"au",
			b"\
				\x08\x00\x00\x00\
				\x04\x03\x02\x01\
				\x08\x07\x06\x05\
			",
			super::Variant::ArrayU32((&[
				0x01020304_u32,
				0x05060708_u32,
			][..]).into()),
		);

		test(
			"yau",
			b"\
				\x05\
				\x00\x00\x00\
				\x08\x00\x00\x00\
				\x04\x03\x02\x01\
				\x08\x07\x06\x05\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::ArrayU32((&[
						0x01020304_u32,
						0x05060708_u32,
					][..]).into()),
				][..]).into(),
			},
		);

		test(
			"ay",
			b"\
				\x08\x00\x00\x00\
				\x01\x02\x03\x04\
				\x05\x06\x07\x08\
			",
			super::Variant::ArrayU8((&[
				0x01, 0x02, 0x03, 0x04,
				0x05, 0x06, 0x07, 0x08,
			][..]).into()),
		);

		test(
			"yay",
			b"\
				\x05\
				\x00\x00\x00\
				\x08\x00\x00\x00\
				\x01\x02\x03\x04\
				\x05\x06\x07\x08\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::ArrayU8((&[
						0x01, 0x02, 0x03, 0x04,
						0x05, 0x06, 0x07, 0x08,
					][..]).into()),
				][..]).into(),
			},
		);

		test(
			"b",
			b"\x00\x00\x00\x00",
			super::Variant::Bool(false),
		);

		test(
			"b",
			b"\x01\x00\x00\x00",
			super::Variant::Bool(true),
		);

		test(
			"yb",
			b"\
				\x05\
				\x00\x00\x00\
				\x01\x00\x00\x00\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::Bool(true),
				][..]).into(),
			},
		);

		test(
			"a{qs}",
			b"\
				\x3D\x00\x00\x00\
				\x00\x00\x00\x00\
				\x02\x01\
				\x00\x00\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\
				\x04\x03\
				\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Array {
				element_signature: crate::Signature::DictEntry {
					key: Box::new(crate::Signature::U16),
					value: Box::new(crate::Signature::String),
				},
				elements: (&[
					super::Variant::DictEntry {
						key: (&super::Variant::U16(0x0102)).into(),
						value: (&super::Variant::String("/org/freedesktop/DBus".into())).into(),
					},
					super::Variant::DictEntry {
						key: (&super::Variant::U16(0x0304)).into(),
						value: (&super::Variant::String("org.freedesktop.DBus".into())).into(),
					},
				][..]).into(),
			},
		);

		test(
			"ya{qs}",
			b"\
				\x05\
				\x00\x00\x00\
				\x3D\x00\x00\x00\
				\x02\x01\
				\x00\x00\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\
				\x04\x03\
				\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::Array {
						element_signature: crate::Signature::DictEntry {
							key: Box::new(crate::Signature::U16),
							value: Box::new(crate::Signature::String),
						},
						elements: (&[
							super::Variant::DictEntry {
								key: (&super::Variant::U16(0x0102)).into(),
								value: (&super::Variant::String("/org/freedesktop/DBus".into())).into(),
							},
							super::Variant::DictEntry {
								key: (&super::Variant::U16(0x0304)).into(),
								value: (&super::Variant::String("org.freedesktop.DBus".into())).into(),
							},
						][..]).into(),
					},
				][..]).into(),
			},
		);

		test(
			"d",
			b"\x58\x39\xB4\xC8\x76\xBE\xF3\x3F",
			super::Variant::F64(1.234),
		);

		test(
			"yd",
			b"\
				\x05\
				\x00\x00\x00\x00\x00\x00\x00\
				\x58\x39\xB4\xC8\x76\xBE\xF3\x3F\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::F64(1.234),
				][..]).into(),
			},
		);

		test(
			"g",
			b"\0\0",
			crate::Variant::Signature(crate::Signature::Tuple { elements: vec![] }),
		);

		test(
			"g",
			b"\x01s\0",
			super::Variant::Signature(crate::Signature::String),
		);

		test(
			"g",
			b"\x05(aus)\0",
			super::Variant::Signature(crate::Signature::Struct {
				fields: (&[
					crate::Signature::Array {
						element: Box::new(crate::Signature::U32),
					},
					crate::Signature::String,
				][..]).into(),
			}),
		);

		test(
			"g",
			b"\x05a{us}\0",
			super::Variant::Signature(
				crate::Signature::Array {
					element: Box::new(crate::Signature::DictEntry {
						key: Box::new(crate::Signature::U32),
						value: Box::new(crate::Signature::String),
					}),
				},
			),
		);

		test(
			"yg",
			b"\
				\x05\
				\x01s\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::Signature(crate::Signature::String),
				][..]).into(),
			},
		);

		test(
			"h",
			b"\x04\x03\x02\x01",
			super::Variant::UnixFd(crate::UnixFd(0x01020304)),
		);

		test(
			"yh",
			b"\
				\x05\
				\x00\x00\x00\
				\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::UnixFd(crate::UnixFd(0x01020304)),
				][..]).into(),
			},
		);

		test(
			"i",
			b"\x00\x00\x00\x01",
			super::Variant::I32(0x01000000),
		);

		test(
			"yi",
			b"\
				\x05\
				\x00\x00\x00\
				\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::I32(0x01020304),
				][..]).into(),
			},
		);

		test(
			"n",
			b"\x02\x01",
			super::Variant::I16(0x0102),
		);

		test(
			"yn",
			b"\
				\x05\
				\x00\
				\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::I16(0x0102),
				][..]).into(),
			},
		);

		test(
			"o",
			b"\x15\x00\x00\x00/org/freedesktop/DBus\0",
			super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
		);

		test(
			"yo",
			b"\
				\x05\
				\x00\x00\x00\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
				][..]).into(),
			},
		);

		test(
			"q",
			b"\x02\x01",
			super::Variant::U16(0x0102),
		);

		test(
			"yq",
			b"\
				\x05\
				\x00\
				\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::U16(0x0102),
				][..]).into(),
			},
		);

		test(
			"s",
			b"\x00\x00\x00\x00\0",
			super::Variant::String("".into()),
		);

		test(
			"s",
			b"\x14\x00\x00\x00org.freedesktop.DBus\0",
			super::Variant::String("org.freedesktop.DBus".into()),
		);

		test(
			"ys",
			b"\
				\x05\
				\x00\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::String("org.freedesktop.DBus".into()),
				][..]).into(),
			},
		);

		test(
			"t",
			b"\x08\x07\x06\x05\x04\x03\x02\x01",
			super::Variant::U64(0x01020304_05060708),
		);

		test(
			"yt",
			b"\
				\x05\
				\x00\x00\x00\x00\x00\x00\x00\
				\x08\x07\x06\x05\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::U64(0x01020304_05060708),
				][..]).into(),
			},
		);

		test(
			"u",
			b"\x04\x03\x02\x01",
			super::Variant::U32(0x01020304),
		);

		test(
			"yu",
			b"\
				\x05\
				\x00\x00\x00\
				\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::U32(0x01020304),
				][..]).into(),
			},
		);

		test(
			"v",
			b"\x01s\0\x00\x14\x00\x00\x00org.freedesktop.DBus\0",
			super::Variant::Variant((&
				super::Variant::String("org.freedesktop.DBus".into())
			).into()),
		);

		test(
			"v",
			b"\
				\x01g\0\
				\0\0\
			",
			super::Variant::Variant((&crate::Variant::Signature(crate::Signature::Tuple { elements: vec![] })).into()),
		);

		test(
			"yv",
			b"\
				\x05\
				\x01s\0\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::Variant((&
						super::Variant::String("org.freedesktop.DBus".into())
					).into()),
				][..]).into(),
			},
		);

		test(
			"x",
			b"\x08\x07\x06\x05\x04\x03\x02\x01",
			super::Variant::I64(0x01020304_05060708),
		);

		test(
			"yx",
			b"\
				\x05\
				\x00\x00\x00\x00\x00\x00\x00\
				\x08\x07\x06\x05\x04\x03\x02\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::I64(0x01020304_05060708),
				][..]).into(),
			},
		);

		test(
			"y",
			b"\x01",
			super::Variant::U8(0x01),
		);

		test(
			"yy",
			b"\
				\x05\
				\x01\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::U8(0x01),
				][..]).into(),
			},
		);

		test(
			"(uos)",
			b"\
				\x04\x03\x02\x01\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Struct {
				fields: (&[
					super::Variant::U32(0x01020304),
					super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
					super::Variant::String("org.freedesktop.DBus".into()),
				][..]).into(),
			},
		);

		test(
			"(uuo(sou)s)",
			b"\
				\x04\x03\x02\x01\
				\x08\x07\x06\x05\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\x00\x00\x00\x00\
					\x14\x00\x00\x00org.freedesktop.DBus\0\
					\x00\x00\x00\
					\x15\x00\x00\x00/org/freedesktop/DBus\0\
					\x00\x00\
					\x04\x03\x02\x01\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Struct {
				fields: (&[
					super::Variant::U32(0x01020304),
					super::Variant::U32(0x05060708),
					super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
					super::Variant::Struct {
						fields: (&[
							super::Variant::String("org.freedesktop.DBus".into()),
							super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
							super::Variant::U32(0x01020304),
						][..]).into(),
					},
					super::Variant::String("org.freedesktop.DBus".into()),
				][..]).into(),
			},
		);

		test(
			"y(uos)",
			b"\
				\x05\
				\x00\x00\x00\x00\x00\x00\x00\
				\x04\x03\x02\x01\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(0x05),
					super::Variant::Struct {
						fields: (&[
							super::Variant::U32(0x01020304),
							super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
							super::Variant::String("org.freedesktop.DBus".into()),
						][..]).into(),
					},
				][..]).into(),
			},
		);

		test(
			"uos",
			b"\
				\x04\x03\x02\x01\
				\x15\x00\x00\x00/org/freedesktop/DBus\0\
				\x00\x00\
				\x14\x00\x00\x00org.freedesktop.DBus\0\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U32(0x01020304),
					super::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
					super::Variant::String("org.freedesktop.DBus".into()),
				][..]).into(),
			},
		);

		test(
			"a(u)y",
			b"\
				\x00\x00\x00\x00\
				\x00\x00\x00\x00\
				\x03\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::Array {
						element_signature: crate::Signature::Struct {
							fields: vec![
								crate::Signature::U32,
							],
						},
						elements: (&[][..]).into(),
					},
					super::Variant::U8(3),
				][..]).into(),
			},
		);

		test(
			"ya(u)y",
			b"\
				\x05\
				\x00\x00\x00\
				\x00\x00\x00\x00\
				\x03\
			",
			super::Variant::Tuple {
				elements: (&[
					super::Variant::U8(5),
					super::Variant::Array {
						element_signature: crate::Signature::Struct {
							fields: vec![
								crate::Signature::U32,
							],
						},
						elements: (&[][..]).into(),
					},
					super::Variant::U8(3),
				][..]).into(),
			},
		);
	}
}
