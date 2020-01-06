impl<'de, 'a> serde::Deserializer<'de> for crate::types::Variant<'de> {
	type Error = crate::de::DeserializeError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
		#[allow(clippy::match_same_arms)]
		match self {
			crate::types::Variant::Array { element_signature, elements } =>
				// Special-case `Array { element_signature: DictEntry { ... } }` to call visit_map,
				// since that's what serde's Deserialize impls for `std::collections::*Map` want.
				if let crate::types::Signature::DictEntry { key: _, value: _ } = element_signature {
					let entries =
						elements.into_owned()
						.into_iter()
						.map(|element|
							if let crate::types::Variant::DictEntry { key, value } = element {
								Ok((key.into_owned(), value.into_owned()))
							}
							else {
								Err(crate::de::DeserializeError::ArrayElementDoesntMatchSignature {
									expected: element_signature.clone(),
									actual: element.inner_signature(),
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

			crate::types::Variant::ArrayBool(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::Bool))),

			crate::types::Variant::ArrayF64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::F64))),

			crate::types::Variant::ArrayI16(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::I16))),

			crate::types::Variant::ArrayI32(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::I32))),

			crate::types::Variant::ArrayI64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::I64))),

			crate::types::Variant::ArrayString(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::String))),

			crate::types::Variant::ArrayU8(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::U8))),

			crate::types::Variant::ArrayU16(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::U16))),

			crate::types::Variant::ArrayU32(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::U32))),

			crate::types::Variant::ArrayU64(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::U64))),

			crate::types::Variant::ArrayUnixFd(elements) =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter().map(crate::types::Variant::UnixFd))),

			crate::types::Variant::Bool(value) =>
				visitor.visit_bool(value),

			crate::types::Variant::DictEntry { key, value } =>
				visitor.visit_seq(SeqAccess(std::iter::once(key.into_owned()).chain(std::iter::once(value.into_owned())))),

			crate::types::Variant::F64(value) =>
				visitor.visit_f64(value),

			crate::types::Variant::I16(value) =>
				visitor.visit_i16(value),

			crate::types::Variant::I32(value) =>
				visitor.visit_i32(value),

			crate::types::Variant::I64(value) =>
				visitor.visit_i64(value),

			crate::types::Variant::ObjectPath(crate::types::ObjectPath(value)) =>
				crate::types::Variant::String(value).deserialize_any(visitor),

			crate::types::Variant::Signature(value) =>
				crate::types::Variant::String(value.to_string().into()).deserialize_any(visitor),

			crate::types::Variant::String(value) => match value {
				std::borrow::Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
				std::borrow::Cow::Owned(value) => visitor.visit_string(value),
			},

			crate::types::Variant::Struct { fields } =>
				visitor.visit_seq(SeqAccess(fields.into_owned().into_iter())),

			crate::types::Variant::Tuple { elements } =>
				visitor.visit_seq(SeqAccess(elements.into_owned().into_iter())),

			crate::types::Variant::U8(value) =>
				visitor.visit_u8(value),

			crate::types::Variant::U16(value) =>
				visitor.visit_u16(value),

			crate::types::Variant::U32(value) =>
				visitor.visit_u32(value),

			crate::types::Variant::U64(value) =>
				visitor.visit_u64(value),

			crate::types::Variant::UnixFd(crate::types::UnixFd(value)) =>
				visitor.visit_u32(value),

			crate::types::Variant::Variant(value) =>
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

impl<'de, I> serde::de::SeqAccess<'de> for SeqAccess<I> where I: Iterator<Item = crate::types::Variant<'de>> {
	type Error = crate::de::DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: serde::de::DeserializeSeed<'de> {
		self.0.next()
			.map(|element| seed.deserialize(element))
			.transpose()
	}
}

struct MapAccess<'de, I> {
	entries: I,
	next_value: Option<crate::types::Variant<'de>>,
}

impl<'de, I> serde::de::MapAccess<'de> for MapAccess<'de, I> where I: Iterator<Item = Result<(crate::types::Variant<'de>, crate::types::Variant<'de>), crate::de::DeserializeError>> {
	type Error = crate::de::DeserializeError;

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

#[cfg(test)]
mod tests {
	#[test]
	fn test_variant_deserializer() {
		fn test<T>(
			variant: crate::types::Variant<'_>,
			expected_deserialize: T,
		) where T: std::fmt::Debug + PartialEq + serde::de::DeserializeOwned {
			let actual_deserialize: T = serde::de::Deserialize::deserialize(variant).unwrap();
			assert_eq!(expected_deserialize, actual_deserialize);
		}

		test(
			crate::types::Variant::Array {
				element_signature: crate::types::Signature::DictEntry {
					key: Box::new(crate::types::Signature::String).into(),
					value: Box::new(crate::types::Signature::U32).into(),
				},
				elements: vec![
					crate::types::Variant::DictEntry {
						key: (&crate::types::Variant::String("foo".into())).into(),
						value: (&crate::types::Variant::U32(3)).into(),
					},
					crate::types::Variant::DictEntry {
						key: (&crate::types::Variant::String("bar".into())).into(),
						value: (&crate::types::Variant::U32(5)).into(),
					},
				].into()
			},
			vec![
				("foo".to_owned(), 3),
				("bar".to_owned(), 5),
			].into_iter().collect::<std::collections::BTreeMap<_, _>>(),
		);

		test(
			crate::types::Variant::Array {
				element_signature: crate::types::Signature::Struct {
					fields: vec![
						crate::types::Signature::String,
						crate::types::Signature::U32,
					],
				},
				elements: vec![
					crate::types::Variant::Struct {
						fields: (&[
							crate::types::Variant::String("abc".into()),
							crate::types::Variant::U32(3),
						][..]).into(),
					},
					crate::types::Variant::Struct {
						fields: (&[
							crate::types::Variant::String("def".into()),
							crate::types::Variant::U32(5),
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
			crate::types::Variant::ArrayU32((&[0x01020304, 0x05060708][..]).into()),
			vec![0x01020304_u32, 0x05060708],
		);

		test(
			crate::types::Variant::Bool(true),
			true,
		);

		test(
			crate::types::Variant::DictEntry {
				key: (&crate::types::Variant::String("foo".into())).into(),
				value: (&crate::types::Variant::U32(3)).into(),
			},
			("foo".to_owned(), 3),
		);

		test(
			crate::types::Variant::ObjectPath(crate::types::ObjectPath("/org/freedesktop/DBus".into())),
			"/org/freedesktop/DBus".to_owned(),
		);

		test(
			crate::types::Variant::Signature(crate::types::Signature::Array { element: Box::new(crate::types::Signature::U8) }),
			"ay".to_owned(),
		);

		test(
			crate::types::Variant::U32(0x01020304),
			0x01020304_u32,
		);
	}
}
