/// A variant. It can store any kind of data type that D-Bus supports.
///
/// Note that `Variant` does not impl `serde::Deserialize` since it needs to know the signature to be able to deserialize itself.
/// Instead, use [`VariantDeserializeSeed`] to deserialize a `Variant`.
#[derive(Clone, Debug, PartialEq)]
pub enum Variant<'a> {
	/// An array of variants. All variants must have the same signature as `element_signature`.
	///
	/// Simpler arrays will be deserialized as the other `Array*` variants.
	/// For example, byte arrays (`ay`) will always be deserialized as `ArrayU8`.
	Array {
		element_signature: crate::types::Signature,
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

	ArrayUnixFd(std::borrow::Cow<'a, [crate::types::UnixFd]>),

	Bool(bool),

	DictEntry {
		key: crate::std2::CowRef<'a, Variant<'a>>,
		value: crate::std2::CowRef<'a, Variant<'a>>,
	},

	F64(f64),

	I16(i16),

	I32(i32),

	I64(i64),

	ObjectPath(crate::types::ObjectPath<'a>),

	Signature(crate::types::Signature),

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

	UnixFd(crate::types::UnixFd),

	Variant(crate::std2::CowRef<'a, Variant<'a>>),
}

impl<'a> Variant<'a> {
	/// Convenience function to view this `Variant` as a `&[Variant]` if it's an array and its elements have the given signature.
	pub fn as_array<'b>(&'b self, expected_element_signature: &crate::types::Signature) -> Option<&'b [Variant<'a>]> {
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

	pub(crate) fn inner_signature(&self) -> crate::types::Signature {
		match self {
			Variant::Array { element_signature, elements: _ } =>
				crate::types::Signature::Array { element: Box::new(element_signature.clone()) },

			Variant::ArrayBool(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::Bool) },

			Variant::ArrayF64(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::F64) },

			Variant::ArrayI16(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::I16) },

			Variant::ArrayI32(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::I32) },

			Variant::ArrayI64(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::I64) },

			Variant::ArrayString(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::String) },

			Variant::ArrayU8(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::U8) },

			Variant::ArrayU16(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::U16) },

			Variant::ArrayU32(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::U32) },

			Variant::ArrayU64(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::U64) },

			Variant::ArrayUnixFd(_) =>
				crate::types::Signature::Array { element: Box::new(crate::types::Signature::UnixFd) },

			Variant::Bool(_) =>
				crate::types::Signature::Bool,

			Variant::DictEntry { key, value } =>
				crate::types::Signature::DictEntry {
					key: Box::new(key.inner_signature()),
					value: Box::new(value.inner_signature()),
				},

			Variant::F64(_) =>
				crate::types::Signature::F64,

			Variant::I16(_) =>
				crate::types::Signature::I16,

			Variant::I32(_) =>
				crate::types::Signature::I32,

			Variant::I64(_) =>
				crate::types::Signature::I64,

			Variant::ObjectPath(_) =>
				crate::types::Signature::ObjectPath,

			Variant::Signature(_) =>
				crate::types::Signature::Signature,

			Variant::String(_) =>
				crate::types::Signature::String,

			Variant::Struct { fields } =>
				crate::types::Signature::Struct { fields: fields.iter().map(Variant::inner_signature).collect() },

			Variant::Tuple { elements } =>
				crate::types::Signature::Tuple { elements: elements.iter().map(Variant::inner_signature).collect() },

			Variant::U8(_) =>
				crate::types::Signature::U8,

			Variant::U16(_) =>
				crate::types::Signature::U16,

			Variant::U32(_) =>
				crate::types::Signature::U32,

			Variant::U64(_) =>
				crate::types::Signature::U64,

			Variant::UnixFd(_) =>
				crate::types::Signature::UnixFd,

			Variant::Variant(_) =>
				crate::types::Signature::Variant,
		}
	}
}

impl serde::Serialize for Variant<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		use serde::ser::{SerializeStruct, SerializeTuple};

		match self {
			Variant::Array { element_signature, elements } =>
				(crate::types::Slice { inner: elements, alignment: element_signature.alignment() }).serialize(serializer),

			Variant::ArrayBool(elements) =>
				(crate::types::Slice { inner: elements, alignment: 4 }).serialize(serializer),

			Variant::ArrayF64(elements) =>
				(crate::types::Slice { inner: elements, alignment: 8 }).serialize(serializer),

			Variant::ArrayI16(elements) =>
				(crate::types::Slice { inner: elements, alignment: 2 }).serialize(serializer),

			Variant::ArrayI32(elements) =>
				(crate::types::Slice { inner: elements, alignment: 4 }).serialize(serializer),

			Variant::ArrayI64(elements) =>
				(crate::types::Slice { inner: elements, alignment: 8 }).serialize(serializer),

			Variant::ArrayString(elements) =>
				(crate::types::Slice { inner: elements, alignment: 4 }).serialize(serializer),

			Variant::ArrayU8(elements) =>
				(crate::types::Slice { inner: elements, alignment: 1 }).serialize(serializer),

			Variant::ArrayU16(elements) =>
				(crate::types::Slice { inner: elements, alignment: 2 }).serialize(serializer),

			Variant::ArrayU32(elements) =>
				(crate::types::Slice { inner: elements, alignment: 4 }).serialize(serializer),

			Variant::ArrayU64(elements) =>
				(crate::types::Slice { inner: elements, alignment: 8 }).serialize(serializer),

			Variant::ArrayUnixFd(elements) =>
				(crate::types::Slice { inner: elements, alignment: 4 }).serialize(serializer),

			Variant::Bool(value) =>
				value.serialize(serializer),

			Variant::DictEntry { key, value } => {
				let mut serializer = serializer.serialize_struct("", 2)?;
				serializer.serialize_field("key", &**key)?;
				serializer.serialize_field("value", &**value)?;
				serializer.end()
			},

			Variant::F64(value) =>
				value.serialize(serializer),

			Variant::I16(value) =>
				value.serialize(serializer),

			Variant::I32(value) =>
				value.serialize(serializer),

			Variant::I64(value) =>
				value.serialize(serializer),

			Variant::ObjectPath(value) =>
				value.serialize(serializer),

			Variant::Signature(value) =>
				value.serialize(serializer),

			Variant::String(value) =>
				value.serialize(serializer),

			Variant::Struct { fields } => {
				let mut serializer = serializer.serialize_struct("", fields.len())?;
				for field in &**fields {
					serializer.serialize_field("", field)?;
				}
				serializer.end()
			},

			Variant::Tuple { elements } => {
				let mut serializer = serializer.serialize_tuple(elements.len())?;
				for element in &**elements {
					serializer.serialize_element(element)?;
				}
				serializer.end()
			},

			Variant::U8(value) =>
				value.serialize(serializer),

			Variant::U16(value) =>
				value.serialize(serializer),

			Variant::U32(value) =>
				value.serialize(serializer),

			Variant::U64(value) =>
				value.serialize(serializer),

			Variant::UnixFd(value) =>
				value.serialize(serializer),

			Variant::Variant(value) => {
				let mut serializer = serializer.serialize_tuple(2)?;
				let signature = value.inner_signature();
				serializer.serialize_element(&signature)?;
				serializer.serialize_element(&**value)?;
				serializer.end()
			},
		}
	}
}

/// Used to deserialize a [`Variant`] using its [`serde::de::DeserializeSeed`] impl.
#[derive(Debug)]
pub struct VariantDeserializeSeed<'input, 'output>(&'input crate::types::Signature, std::marker::PhantomData<fn() -> Variant<'output>>);

impl<'input, 'output> VariantDeserializeSeed<'input, 'output> {
	/// Construct a `VariantDeserializeSeed` that will deserialize a [`Variant`] of the given signature.
	pub fn new(signature: &'input crate::types::Signature) -> Self {
		VariantDeserializeSeed(signature, Default::default())
	}
}

impl<'de, 'input, 'output> serde::de::DeserializeSeed<'de> for VariantDeserializeSeed<'input, 'output> {
	type Value = Variant<'output>;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
		struct Visitor<'input, 'output>(&'input crate::types::Signature, std::marker::PhantomData<fn() -> Variant<'output>>);

		impl<'de, 'input, 'output> serde::de::Visitor<'de> for Visitor<'input, 'output> {
			type Value = Variant<'output>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("variant")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
				match self.0 {
					crate::types::Signature::Array { element } => {
						let element_seed = ArrayDeserializeSeed((&**element).clone(), self.1);
						let value = seq.next_element_seed(element_seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(value)
					},

					crate::types::Signature::Bool => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::Bool(value))
					},

					crate::types::Signature::DictEntry { key, value } => {
						let () = seq.next_element_seed(StructDeserializeSeed)?.expect("cannot fail");

						let seed = VariantDeserializeSeed(key, self.1);
						let key = seq.next_element_seed(seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;

						let seed = VariantDeserializeSeed(value, self.1);
						let value = seq.next_element_seed(seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;

						Ok(Variant::DictEntry { key: Box::new(key).into(), value: Box::new(value).into() })
					},

					crate::types::Signature::F64 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::F64(value))
					},

					crate::types::Signature::I16 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::I16(value))
					},

					crate::types::Signature::I32 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::I32(value))
					},

					crate::types::Signature::I64 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::I64(value))
					},

					crate::types::Signature::ObjectPath => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::ObjectPath(value))
					},

					crate::types::Signature::Signature => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::Signature(value))
					},

					crate::types::Signature::String => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::String(value))
					},

					crate::types::Signature::Struct { fields } => {
						let () = seq.next_element_seed(StructDeserializeSeed)?.expect("cannot fail");

						let fields: Result<Vec<_>, _> =
							fields.iter()
							.map(|field| {
								let seed = VariantDeserializeSeed(field, self.1);
								let field = seq.next_element_seed(seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
								Ok(field)
							})
							.collect();
						let fields = fields?;
						Ok(Variant::Struct { fields: fields.into() })
					},

					crate::types::Signature::Tuple { elements } => {
						let elements: Result<Vec<_>, _> =
							elements.iter()
							.map(|element| {
								let seed = VariantDeserializeSeed(element, self.1);
								let element = seq.next_element_seed(seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
								Ok(element)
							})
							.collect();
						let elements = elements?;
						Ok(Variant::Tuple { elements: elements.into() })
					},

					crate::types::Signature::U8 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::U8(value))
					},

					crate::types::Signature::U16 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::U16(value))
					},

					crate::types::Signature::U32 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::U32(value))
					},

					crate::types::Signature::U64 => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::U64(value))
					},

					crate::types::Signature::UnixFd => {
						let value = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::UnixFd(value))
					},

					crate::types::Signature::Variant => {
						let signature: crate::types::Signature = seq.next_element()?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						let seed = VariantDeserializeSeed(&signature, self.1);
						let value: Variant<'output> = seq.next_element_seed(seed)?.ok_or_else(|| serde::de::Error::missing_field("value"))?;
						Ok(Variant::Variant(Box::new(value).into()))
					},
				}
			}
		}

		struct ArrayDeserializeSeed<'output>(crate::types::Signature, std::marker::PhantomData<fn() -> Variant<'output>>);

		impl<'de, 'output> serde::de::DeserializeSeed<'de> for ArrayDeserializeSeed<'output> {
			type Value = Variant<'output>;

			fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
				struct Visitor<'output>(crate::types::Signature, std::marker::PhantomData<fn() -> Variant<'output>>);

				impl<'de, 'output> serde::de::Visitor<'de> for Visitor<'output> {
					type Value = Variant<'output>;

					fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
						write!(formatter, "Array({:?})", self.0)
					}

					fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
						match self.0 {
							crate::types::Signature::Bool => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayBool(elements.into()))
							},

							crate::types::Signature::F64 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayF64(elements.into()))
							},

							crate::types::Signature::I16 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayI16(elements.into()))
							},

							crate::types::Signature::I32 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayI32(elements.into()))
							},

							crate::types::Signature::I64 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayI64(elements.into()))
							},

							crate::types::Signature::String => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayString(elements.into()))
							},

							crate::types::Signature::U8 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayU8(elements.into()))
							},

							crate::types::Signature::U16 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayU16(elements.into()))
							},

							crate::types::Signature::U32 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayU32(elements.into()))
							},

							crate::types::Signature::U64 => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayU64(elements.into()))
							},

							crate::types::Signature::UnixFd => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element()? {
									elements.push(element);
								}
								Ok(Variant::ArrayUnixFd(elements.into()))
							},

							element_signature => {
								let mut elements = vec![];
								while let Some(element) = seq.next_element_seed(VariantDeserializeSeed(&element_signature, self.1))? {
									elements.push(element);
								}
								Ok(Variant::Array { element_signature, elements: elements.into() })
							},
						}
					}
				}

				deserializer.deserialize_seq(Visitor(self.0, self.1))
			}
		}

		// Instantiated once when beginning to deserialize a struct. Doesn't actually deserialize anything, but enforces struct padding.
		struct StructDeserializeSeed;

		impl<'de> serde::de::DeserializeSeed<'de> for StructDeserializeSeed {
			type Value = ();

			fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
				struct Visitor;

				impl<'de> serde::de::Visitor<'de> for Visitor {
					type Value = ();

					fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
						formatter.write_str("")
					}

					fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
						Ok(())
					}
				}

				deserializer.deserialize_struct("", &[], Visitor)
			}
		}

		deserializer.deserialize_tuple(0, Visitor(&*self.0, self.1))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_variant_serde() {
		fn test(
			signature: &str,
			expected_serialized: &[u8],
			expected_variant: super::Variant<'_>,
		) {
			// Hard-coded test inputs are for little-endian architecture.
			assert!(cfg!(target_endian = "little"));

			let signature: crate::types::Signature = signature.parse().unwrap();

			let deserialize_seed = crate::types::VariantDeserializeSeed::new(&signature);

			let mut deserializer = crate::de::Deserializer::new(expected_serialized, 0, crate::Endianness::Little);
			let actual_variant: super::Variant<'_> = serde::de::DeserializeSeed::deserialize(deserialize_seed, &mut deserializer).unwrap();
			assert_eq!(expected_variant, actual_variant);

			let mut actual_serialized = vec![];
			let mut serializer = crate::ser::Serializer::new(&mut actual_serialized, crate::Endianness::Little);
			serde::Serialize::serialize(&actual_variant, &mut serializer).unwrap();
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
				element_signature: crate::types::Signature::DictEntry {
					key: Box::new(crate::types::Signature::U16),
					value: Box::new(crate::types::Signature::String),
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
						element_signature: crate::types::Signature::DictEntry {
							key: Box::new(crate::types::Signature::U16),
							value: Box::new(crate::types::Signature::String),
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
			b"\x01s\0",
			super::Variant::Signature(crate::types::Signature::String),
		);

		test(
			"g",
			b"\x05(aus)\0",
			super::Variant::Signature(crate::types::Signature::Struct {
				fields: (&[
					crate::types::Signature::Array {
						element: Box::new(crate::types::Signature::U32),
					},
					crate::types::Signature::String,
				][..]).into(),
			}),
		);

		test(
			"g",
			b"\x05a{us}\0",
			super::Variant::Signature(
				crate::types::Signature::Array {
					element: Box::new(crate::types::Signature::DictEntry {
						key: Box::new(crate::types::Signature::U32),
						value: Box::new(crate::types::Signature::String),
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
					super::Variant::Signature(crate::types::Signature::String),
				][..]).into(),
			},
		);

		test(
			"h",
			b"\x04\x03\x02\x01",
			super::Variant::UnixFd(crate::types::UnixFd(0x01020304)),
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
					super::Variant::UnixFd(crate::types::UnixFd(0x01020304)),
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
			super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
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
					super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
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
					super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
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
					super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
					super::Variant::Struct {
						fields: (&[
							super::Variant::String("org.freedesktop.DBus".into()),
							super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
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
							super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
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
					super::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
					super::Variant::String("org.freedesktop.DBus".into()),
				][..]).into(),
			},
		);
	}
}
