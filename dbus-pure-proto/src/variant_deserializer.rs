impl<'de, 'a> serde::Deserializer<'de> for crate::Variant<'de> {
	type Error = VariantDeserializeError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		#[allow(clippy::match_same_arms)]
		match self {
			crate::Variant::Array { element_signature, elements } =>
				// Special-case `Array { element_signature: DictEntry { ... } }` to call visit_map,
				// since that's what serde's Deserialize impls for `std::collections::*Map` want.
				if let crate::Signature::DictEntry { key: _, value: _ } = element_signature {
					let entries =
						elements.into_owned()
						.into_iter()
						.map(|element|
							if let crate::Variant::DictEntry { key, value } = element {
								Ok((key.into_owned(), value.into_owned()))
							}
							else {
								Err(VariantDeserializeError::InvalidValue {
									expected: format!("array element with signature {}", element_signature).into(),
									actual: format!("array element with signature {}", element.inner_signature()),
								})
							});
					visitor.visit_map(MapAccess {
						entries,
						next_value: None,
					})
				}
				else {
					visitor.visit_seq(SeqAccess(elements.into_owned().into_iter()))
				},

			crate::Variant::ArrayBool(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::Bool))),

			crate::Variant::ArrayF64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::F64))),

			crate::Variant::ArrayI16(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::I16))),

			crate::Variant::ArrayI32(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::I32))),

			crate::Variant::ArrayI64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::I64))),

			crate::Variant::ArrayString(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::String))),

			crate::Variant::ArrayU8(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::U8))),

			crate::Variant::ArrayU16(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::U16))),

			crate::Variant::ArrayU32(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::U32))),

			crate::Variant::ArrayU64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::U64))),

			crate::Variant::ArrayUnixFd(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::Variant::UnixFd))),

			crate::Variant::Bool(value) =>
				visitor.visit_bool(value),

			crate::Variant::DictEntry { key, value } =>
				visitor.visit_seq(SeqAccess(std::iter::once(key.into_owned()).chain(std::iter::once(value.into_owned())))),

			crate::Variant::F64(value) =>
				visitor.visit_f64(value),

			crate::Variant::I16(value) =>
				visitor.visit_i16(value),

			crate::Variant::I32(value) =>
				visitor.visit_i32(value),

			crate::Variant::I64(value) =>
				visitor.visit_i64(value),

			crate::Variant::ObjectPath(crate::ObjectPath(value)) =>
				crate::Variant::String(value).deserialize_any(visitor),

			crate::Variant::Signature(value) =>
				crate::Variant::String(value.to_string().into()).deserialize_any(visitor),

			crate::Variant::String(value) => match value {
				std::borrow::Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
				std::borrow::Cow::Owned(value) => visitor.visit_string(value),
			},

			crate::Variant::Struct { fields } =>
				visitor.visit_seq(SeqAccess(fields.into_owned().into_iter())),

			crate::Variant::Tuple { elements } =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter())),

			crate::Variant::U8(value) =>
				visitor.visit_u8(value),

			crate::Variant::U16(value) =>
				visitor.visit_u16(value),

			crate::Variant::U32(value) =>
				visitor.visit_u32(value),

			crate::Variant::U64(value) =>
				visitor.visit_u64(value),

			crate::Variant::UnixFd(crate::UnixFd(value)) =>
				visitor.visit_u32(value),

			crate::Variant::Variant(value) =>
				value.into_owned().deserialize_any(visitor),
		}
	}

	serde::forward_to_deserialize_any! {
		bool
		i8 i16 i32 i64 i128
		u8 u16 u32 u64 u128
		f32 f64
		char
		str string
		bytes byte_buf
		option
		unit unit_struct
		newtype_struct
		seq tuple tuple_struct
		map
		struct
		enum
		identifier
		ignored_any
	}

	fn is_human_readable(&self) -> bool {
		false
	}
}

struct SeqAccess<I>(I);

impl<'de, I> serde::de::SeqAccess<'de> for SeqAccess<I> where I: Iterator<Item = crate::Variant<'de>> {
	type Error = VariantDeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: serde::de::DeserializeSeed<'de> {
		self.0.next()
			.map(|element| seed.deserialize(element))
			.transpose()
	}
}

struct MapAccess<'de, I> {
	entries: I,
	next_value: Option<crate::Variant<'de>>,
}

impl<'de, I> serde::de::MapAccess<'de> for MapAccess<'de, I> where I: Iterator<Item = Result<(crate::Variant<'de>, crate::Variant<'de>), VariantDeserializeError>> {
	type Error = VariantDeserializeError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> where K: serde::de::DeserializeSeed<'de> {
		let (key, value) = match self.entries.next() {
			Some(entry) => entry?,
			None => return Ok(None),
		};
		self.next_value = Some(value);
		Ok(Some(seed.deserialize(key)?))
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error> where V: serde::de::DeserializeSeed<'de> {
		seed.deserialize(self.next_value.take().unwrap())
	}
}

/// An error from deserializing a value from a [`crate::Variant`]
#[derive(Debug)]
pub enum VariantDeserializeError {
	Custom(String),
	InvalidValue { expected: std::borrow::Cow<'static, str>, actual: String },
}

impl std::fmt::Display for VariantDeserializeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		#[allow(clippy::match_same_arms)]
		match self {
			VariantDeserializeError::Custom(message) => f.write_str(message),
			VariantDeserializeError::InvalidValue { expected, actual } => write!(f, "expected {} but got {}", expected, actual),
		}
	}
}

impl std::error::Error for VariantDeserializeError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		#[allow(clippy::match_same_arms)]
		match self {
			VariantDeserializeError::Custom(_) => None,
			VariantDeserializeError::InvalidValue { expected: _, actual: _ } => None,
		}
	}
}

impl serde::de::Error for VariantDeserializeError {
	fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
		VariantDeserializeError::Custom(msg.to_string())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_variant_deserializer() {
		fn test<T>(
			variant: crate::Variant<'_>,
			expected_deserialize: T,
		) where T: std::fmt::Debug + PartialEq + serde::de::DeserializeOwned {
			let actual_deserialize: T = serde::de::Deserialize::deserialize(variant).unwrap();
			assert_eq!(expected_deserialize, actual_deserialize);
		}

		test(
			crate::Variant::Array {
				element_signature: crate::Signature::DictEntry {
					key: Box::new(crate::Signature::String).into(),
					value: Box::new(crate::Signature::U32).into(),
				},
				elements: vec![
					crate::Variant::DictEntry {
						key: (&crate::Variant::String("foo".into())).into(),
						value: (&crate::Variant::U32(3)).into(),
					},
					crate::Variant::DictEntry {
						key: (&crate::Variant::String("bar".into())).into(),
						value: (&crate::Variant::U32(5)).into(),
					},
				].into()
			},
			vec![
				("foo".to_owned(), 3),
				("bar".to_owned(), 5),
			].into_iter().collect::<std::collections::BTreeMap<_, _>>(),
		);

		test(
			crate::Variant::Array {
				element_signature: crate::Signature::Struct {
					fields: vec![
						crate::Signature::String,
						crate::Signature::U32,
					],
				},
				elements: vec![
					crate::Variant::Struct {
						fields: (&[
							crate::Variant::String("abc".into()),
							crate::Variant::U32(3),
						][..]).into(),
					},
					crate::Variant::Struct {
						fields: (&[
							crate::Variant::String("def".into()),
							crate::Variant::U32(5),
						][..]).into(),
					},
				].into()
			},
			{
				#[derive(Debug, PartialEq, serde_derive::Deserialize)]
				struct Foo {
					bar: String,
					baz: u32,
				}

				vec![
					Foo { bar: "abc".to_owned(), baz: 3 },
					Foo { bar: "def".to_owned(), baz: 5 },
				]
			},
		);

		test(
			crate::Variant::ArrayU32((&[0x01020304, 0x05060708][..]).into()),
			vec![0x01020304_u32, 0x05060708],
		);

		test(
			crate::Variant::Bool(true),
			true,
		);

		test(
			crate::Variant::DictEntry {
				key: (&crate::Variant::String("foo".into())).into(),
				value: (&crate::Variant::U32(3)).into(),
			},
			("foo".to_owned(), 3),
		);

		test(
			crate::Variant::ObjectPath(crate::ObjectPath("/org/freedesktop/DBus".into())),
			"/org/freedesktop/DBus".to_owned(),
		);

		test(
			crate::Variant::Signature(crate::Signature::Array { element: Box::new(crate::Signature::U8) }),
			"ay".to_owned(),
		);

		test(
			crate::Variant::U32(0x01020304),
			0x01020304_u32,
		);
	}
}
