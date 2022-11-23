/// A trait to convert a Rust value as a [`crate::Variant`]
///
/// This is useful to allow a value of this type to be serialized into a message body.
///
/// Consider using `#[derive(dbus_pure_macros::AsVariant)]` to implement this trait for your custom struct types,
/// along with `#[derive(serde_derive::Deserialize)]` to be able to deserialize a message body into this type.
pub trait AsVariant {
	/// Get the D-Bus signature of a value of this type.
	fn signature() -> crate::Signature;

	/// Convert this value to a variant.
	fn as_variant<'a>(&'a self) -> crate::Variant<'a>;
}

impl AsVariant for bool {
	fn signature() -> crate::Signature {
		crate::Signature::Bool
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Bool(*self)
	}
}

impl AsVariant for f64 {
	fn signature() -> crate::Signature {
		crate::Signature::F64
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::F64(*self)
	}
}

impl AsVariant for i16 {
	fn signature() -> crate::Signature {
		crate::Signature::I16
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::I16(*self)
	}
}

impl AsVariant for i32 {
	fn signature() -> crate::Signature {
		crate::Signature::I32
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::I32(*self)
	}
}

impl AsVariant for i64 {
	fn signature() -> crate::Signature {
		crate::Signature::I64
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::I64(*self)
	}
}

impl AsVariant for crate::ObjectPath<'_> {
	fn signature() -> crate::Signature {
		crate::Signature::ObjectPath
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::ObjectPath(crate::ObjectPath((&*self.0).into()))
	}
}

impl AsVariant for crate::Signature {
	fn signature() -> crate::Signature {
		crate::Signature::Signature
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Signature(self.clone())
	}
}

impl AsVariant for str {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::String(self.into())
	}
}

impl AsVariant for String {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::String((&**self).into())
	}
}

impl AsVariant for std::borrow::Cow<'_, str> {
	fn signature() -> crate::Signature {
		crate::Signature::String
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::String((&**self).into())
	}
}

impl AsVariant for u8 {
	fn signature() -> crate::Signature {
		crate::Signature::U8
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::U8(*self)
	}
}

impl AsVariant for u16 {
	fn signature() -> crate::Signature {
		crate::Signature::U16
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::U16(*self)
	}
}

impl AsVariant for u32 {
	fn signature() -> crate::Signature {
		crate::Signature::U32
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::U32(*self)
	}
}

impl AsVariant for u64 {
	fn signature() -> crate::Signature {
		crate::Signature::U64
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::U64(*self)
	}
}

impl AsVariant for crate::UnixFd {
	fn signature() -> crate::Signature {
		crate::Signature::UnixFd
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::UnixFd(*self)
	}
}

// Lack of specialization means we can't impl this different for `[u8]` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> AsVariant for [T] where T: AsVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as AsVariant>::signature()),
		}
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Array {
			element_signature: <T as AsVariant>::signature(),
			elements: self.iter().map(AsVariant::as_variant).collect::<Vec<_>>().into(),
		}
	}
}

// Lack of specialization means we can't impl this different for `Cow<'_, [u8]>` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> AsVariant for std::borrow::Cow<'_, [T]> where T: AsVariant, [T]: std::borrow::ToOwned {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as AsVariant>::signature()),
		}
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Array {
			element_signature: <T as AsVariant>::signature(),
			elements: self.iter().map(AsVariant::as_variant).collect::<Vec<_>>().into(),
		}
	}
}

// Lack of specialization means we can't impl this different for `Vec<u8>` etc to use the more efficient `Variant::ArrayU8` etc
impl<T> AsVariant for Vec<T> where T: AsVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(<T as AsVariant>::signature()),
		}
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Array {
			element_signature: <T as AsVariant>::signature(),
			elements: self.iter().map(AsVariant::as_variant).collect::<Vec<_>>().into(),
		}
	}
}

impl<K, V, S> AsVariant for std::collections::HashMap<K, V, S> where K: AsVariant, V: AsVariant {
	fn signature() -> crate::Signature {
		crate::Signature::Array {
			element: Box::new(crate::Signature::DictEntry {
				key: Box::new(<K as AsVariant>::signature()),
				value: Box::new(<V as AsVariant>::signature()),
			}),
		}
	}

	fn as_variant<'a>(&'a self) -> crate::Variant<'a> {
		crate::Variant::Array {
			element_signature: crate::Signature::DictEntry {
				key: Box::new(<K as AsVariant>::signature()),
				value: Box::new(<V as AsVariant>::signature()),
			},
			elements: self.iter().map(|(key, value)| crate::Variant::DictEntry {
				key: crate::std2::CowRef::Owned(Box::new(key.as_variant())),
				value: crate::std2::CowRef::Owned(Box::new(value.as_variant())),
			}).collect::<Vec<_>>().into(),
		}
	}
}
