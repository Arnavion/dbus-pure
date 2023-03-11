/// A trait to convert a Rust value to a [`crate::Variant`]
///
/// This is useful to allow a value of this type to be serialized into a message body.
///
/// Consider using `#[derive(dbus_pure_macros::ToVariant)]` to implement this trait for your custom struct types,
/// along with `#[derive(serde_derive::Deserialize)]` to be able to deserialize a message body into this type.
pub trait ToVariant {
	/// Get the D-Bus signature of a value of this type.
	fn signature() -> crate::Signature;

	/// Convert this value to a variant.
	fn to_variant(&self) -> crate::Variant<'_>;
}

impl ToVariant for bool {
	fn signature() -> crate::Signature {
		crate::Signature::Bool
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Bool(*self)
	}
}

impl ToVariant for f64 {
	fn signature() -> crate::Signature {
		crate::Signature::F64
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::F64(*self)
	}
}

impl ToVariant for i16 {
	fn signature() -> crate::Signature {
		crate::Signature::I16
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::I16(*self)
	}
}

impl ToVariant for i32 {
	fn signature() -> crate::Signature {
		crate::Signature::I32
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::I32(*self)
	}
}

impl ToVariant for i64 {
	fn signature() -> crate::Signature {
		crate::Signature::I64
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::I64(*self)
	}
}

impl ToVariant for crate::ObjectPath<'_> {
	fn signature() -> crate::Signature {
		crate::Signature::ObjectPath
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::ObjectPath(crate::ObjectPath((&*self.0).into()))
	}
}

impl ToVariant for crate::Signature {
	fn signature() -> crate::Signature {
		crate::Signature::Signature
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Signature(self.clone())
	}
}

impl ToVariant for str {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::String(self.into())
	}
}

impl ToVariant for String {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::String((&**self).into())
	}
}

impl ToVariant for std::borrow::Cow<'_, str> {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::String((&**self).into())
	}
}

impl ToVariant for u8 {
	fn signature() -> crate::Signature {
		crate::Signature::U8
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::U8(*self)
	}
}

impl ToVariant for u16 {
	fn signature() -> crate::Signature {
		crate::Signature::U16
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::U16(*self)
	}
}

impl ToVariant for u32 {
	fn signature() -> crate::Signature {
		crate::Signature::U32
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::U32(*self)
	}
}

impl ToVariant for u64 {
	fn signature() -> crate::Signature {
		crate::Signature::U64
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::U64(*self)
	}
}

impl ToVariant for crate::UnixFd {
	fn signature() -> crate::Signature {
		crate::Signature::UnixFd
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::UnixFd(*self)
	}
}

// Lack of specialization means we can't impl this different for `[u8]` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> ToVariant for [T] where T: ToVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as ToVariant>::signature()),
		}
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Array {
			element_signature: <T as ToVariant>::signature(),
			elements: self.iter().map(ToVariant::to_variant).collect::<Vec<_>>().into(),
		}
	}
}

// Lack of specialization means we can't impl this different for `Cow<'_, [u8]>` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> ToVariant for std::borrow::Cow<'_, [T]> where T: ToVariant, [T]: std::borrow::ToOwned {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as ToVariant>::signature()),
		}
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Array {
			element_signature: <T as ToVariant>::signature(),
			elements: self.iter().map(ToVariant::to_variant).collect::<Vec<_>>().into(),
		}
	}
}

// Lack of specialization means we can't impl this different for `Vec<u8>` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> ToVariant for Vec<T> where T: ToVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as ToVariant>::signature()),
		}
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Array {
			element_signature: <T as ToVariant>::signature(),
			elements: self.iter().map(ToVariant::to_variant).collect::<Vec<_>>().into(),
		}
	}
}

impl<K, V, S> ToVariant for std::collections::HashMap<K, V, S> where K: ToVariant, V: ToVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(crate::Signature::DictEntry {
				key: Box::new(<K as ToVariant>::signature()),
				value: Box::new(<V as ToVariant>::signature()),
			}),
		}
	}

	fn to_variant(&self) -> crate::Variant<'_> {
		crate::Variant::Array {
			element_signature: crate::Signature::DictEntry {
				key: Box::new(<K as ToVariant>::signature()),
				value: Box::new(<V as ToVariant>::signature()),
			},
			elements: self.iter().map(|(key, value)| crate::Variant::DictEntry {
				key: crate::std2::CowRef::Owned(Box::new(key.to_variant())),
				value: crate::std2::CowRef::Owned(Box::new(value.to_variant())),
			}).collect::<Vec<_>>().into(),
		}
	}
}
