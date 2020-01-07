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

pub(crate) struct VecDeserializeSeed<T> {
	alignment: usize,
	_pd: std::marker::PhantomData<fn() -> Vec<T>>,
}

impl<T> VecDeserializeSeed<T> {
	pub(crate) fn new(alignment: usize) -> Self {
		VecDeserializeSeed {
			alignment,
			_pd: Default::default(),
		}
	}
}

impl<'de, T> serde::de::DeserializeSeed<'de> for VecDeserializeSeed<T> where T: serde::Deserialize<'de> {
	type Value = Vec<T>;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: serde::Deserializer<'de> {
		struct VecDeserializeVisitor<T>(std::marker::PhantomData<fn() -> Vec<T>>);

		impl<'de, T> serde::de::Visitor<'de> for VecDeserializeVisitor<T> where T: serde::Deserialize<'de> {
			type Value = Vec<T>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("a sequence")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
				let mut result = vec![];

				while let Some(element) = seq.next_element()? {
					result.push(element);
				}

				Ok(result)
			}
		}

		deserializer.deserialize_tuple_struct("", self.alignment, VecDeserializeVisitor(Default::default()))
	}
}
