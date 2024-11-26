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

impl<T> Eq for CowRef<'_, T> where T: Eq {}
