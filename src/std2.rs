//! Extended forms of [`std::borrow::Cow`]

/// Either a borrowed `&'a T` or an owned `Box<T>`
///
/// This exists because `std::borrow::Cow<'a, Foo>` holds an `Owned(Foo)` instead of an `Owned(Box<Foo>)`,
/// which cannot work when the `Cow` is a field of `Foo` itself.
#[derive(Clone, Debug)]
pub enum CowRef<'a, T> {
	Borrowed(&'a T),
	Owned(Box<T>),
}

impl<T> CowRef<'_, T> {
	pub fn into_owned(self) -> T where T: Clone {
		match self {
			CowRef::Borrowed(r) => r.clone(),
			CowRef::Owned(b) => *b,
		}
	}
}

impl<T> std::ops::Deref for CowRef<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			CowRef::Borrowed(r) => r,
			CowRef::Owned(b) => b,
		}
	}
}

impl<'a, T> From<&'a T> for CowRef<'a, T> {
	fn from(r: &'a T) -> Self {
		CowRef::Borrowed(r)
	}
}

impl<T> From<Box<T>> for CowRef<'_, T> {
	fn from(b: Box<T>) -> Self {
		CowRef::Owned(b)
	}
}

impl<T> PartialEq<Self> for CowRef<'_, T> where T: PartialEq<T> {
	fn eq(&self, other: &Self) -> bool {
		**self == **other
	}
}

/// Either a borrowed `&'a [T]` or an owned `Vec<T>`
///
/// This exists because `std::borrow::Cow<'a, [Foo]>` triggers a compiler bug when used as a field of `Foo` itself,
/// as is the case for some types in this crate.
///
/// It can be converted to a regular [`std::borrow::Cow`]`<'_, [T]>` via its [`Into::into`] impl.
///
/// Ref: <https://github.com/rust-lang/rust/issues/38962>
///
/// Ref: <https://github.com/rust-lang/rust/issues/47032>
#[derive(Clone, Debug)]
pub enum CowSlice<'a, T> {
	Borrowed(&'a [T]),
	Owned(Vec<T>),
}

impl<T> CowSlice<'_, T> {
	pub fn into_owned(self) -> Vec<T> where T: Clone {
		match self {
			CowSlice::Borrowed(s) => s.to_owned(),
			CowSlice::Owned(v) => v,
		}
	}
}

impl<T> std::ops::Deref for CowSlice<'_, T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		match self {
			CowSlice::Borrowed(s) => s,
			CowSlice::Owned(v) => v,
		}
	}
}

impl<'a, T> From<&'a [T]> for CowSlice<'a, T> {
	fn from(s: &'a [T]) -> Self {
		CowSlice::Borrowed(s)
	}
}

impl<T> From<Vec<T>> for CowSlice<'_, T> {
	fn from(v: Vec<T>) -> Self {
		CowSlice::Owned(v)
	}
}

impl<'a, T> Into<std::borrow::Cow<'a, [T]>> for CowSlice<'a, T> where T: Clone {
	fn into(self) -> std::borrow::Cow<'a, [T]> {
		match self {
			CowSlice::Borrowed(s) => std::borrow::Cow::Borrowed(s),
			CowSlice::Owned(v) => std::borrow::Cow::Owned(v),
		}
	}
}

impl<T> PartialEq<Self> for CowSlice<'_, T> where T: PartialEq<T> {
	fn eq(&self, other: &Self) -> bool {
		**self == **other
	}
}

impl<'de, T> serde::Deserialize<'de> for CowSlice<'_, T> where T: serde::Deserialize<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		let v: Vec<T> = serde::Deserialize::deserialize(deserializer)?;
		Ok(CowSlice::Owned(v))
	}
}

impl<T> serde::Serialize for CowSlice<'_, T> where T: serde::Serialize {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let s: &[T] = &*self;
		s.serialize(serializer)
	}
}
