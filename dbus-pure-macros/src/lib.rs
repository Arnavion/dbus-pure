#![deny(rust_2018_idioms, warnings)]
#![allow(
	clippy::let_and_return,
)]

//! This crate contains proc macros related to the [`dbus-pure-proto`](https://crates.io/crates/dbus-pure-proto) and
//! [`dbus-pure`](https://crates.io/crates/dbus-pure) crates.

#[allow(unused_extern_crates)] // Needed for stable 1.40.0 but not for nightly
extern crate proc_macro;

mod as_variant;

mod interface;

mod object;

fn run(result: Result<proc_macro2::TokenStream, syn::Error>) -> proc_macro::TokenStream {
	let token_stream = match result {
		Ok(token_stream) => token_stream,
		Err(err) => err.to_compile_error(),
	};
	token_stream.into()
}

trait ResultExt<T> {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error>;
}

impl<T, E> ResultExt<T> for Result<T, E> where E: std::fmt::Display {
	fn spanning(self, spanned: impl quote::ToTokens) -> Result<T, syn::Error> {
		self.map_err(|err| syn::Error::new_spanned(spanned, err))
	}
}

/// Derives `dbus_pure_proto::AsVariant` on the type.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, dbus_pure_macros::AsVariant, serde::Deserialize)]
/// struct Response<'a> {
///     foo: u32,
///     bar: std::borrow::Cow<'a, str>,
/// }
/// ```
#[proc_macro_derive(AsVariant)]
pub fn as_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	run(as_variant::run(input))
}

/// Takes a trait representing a D-Bus interface as input, and emits a trait that can be used to invoke methods using D-Bus.
///
/// ```rust
/// #[dbus_pure_macros::interface("org.freedesktop.DBus")]
/// trait OrgFreeDesktopDbusInterface {
///     #[name = "AddMatch"]
///     fn add_match(rule: &str);
///
///     #[name = "ListNames"]
///     fn list_names() -> Vec<String>;
/// }
/// ```
///
/// The macro modifies the trait definition in these ways:
///
/// - The trait is modified to inherit from `dbus_pure::proto::Object`
///
/// - Every `fn` in the trait is modified to take an additional parameter before any others, of type `&mut dbus_pure::Client`.
///
/// - Every `fn` in the trait is modified to return `Result<TheOriginalReturnType, dbus_pure::MethodCallError>`.
///
/// - Every `fn` in the trait is modified to have a default implementation. This default implementation uses the client
///   to invoke the method and parse its response.
///
/// Thus, the above example will be (approximately) emitted as:
///
/// ```rust,ignore
/// trait OrgFreeDesktopDbusInterface: dbus_pure::proto::Object {
///     fn add_match(client: &mut dbus_pure::Client, rule: &str) -> Result<(), dbus_pure::MethodCallError> {
///         ...
///     }
///
///     fn list_names(client: &mut dbus_pure::Client) -> Result<Vec<String>, dbus_pure::MethodCallError> {
///         ...
///     }
/// }
/// ```
///
/// To use this trait, consider defining an object using the `#[dbus_pure_macros::object]` macro in this crate.
#[proc_macro_attribute]
pub fn interface(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	run(interface::run(attr, item))
}

/// Takes a struct representing a D-Bus object, and implements the given D-Bus interfaces on it.
///
/// ```rust
/// # #[dbus_pure_macros::interface("org.freedesktop.DBus")]
/// # trait OrgFreeDesktopDbusInterface {
/// #     #[name = "AddMatch"]
/// #     fn add_match(rule: &str);
///
/// #     #[name = "ListNames"]
/// #     fn list_names() -> Vec<String>;
/// # }
///
/// #[dbus_pure_macros::object(OrgFreeDesktopDbusInterface)]
/// struct OrgFreeDesktopDbusObject;
/// ```
///
/// Multiple interfaces can be implemented by separating them with `,`
///
/// The macro modifies the struct definition in these ways:
///
/// - The struct is changed to have two members to hold the object's name and object path respectively.
///
/// - The specified traits are implemented on the struct. The interfaces must have been defined using the `#[dbus_pure_macros::interface]` macro.
///
/// Thus, the above example will be (approximately) emitted as:
///
/// ```rust,ignore
/// struct OrgFreeDesktopDbusObject<'a> {
///     name: std::borrow::Cow<'a, str>,
///     path: dbus_pure::proto::ObjectPath<'a>,
/// }
///
/// impl dbus_pure::proto::Object for OrgFreeDesktopDbusObject<'_> {
///     fn name(&self) -> &str {
///         &*self.name
///     }
///
///     fn path(&self) -> dbus_pure::proto::ObjectPath<'_> {
///         dbus_pure::proto::ObjectPath((&*self.path.0).into())
///     }
/// }
///
/// impl OrgFreeDesktopDbusInterface for OrgFreeDesktopDbusObject<'_> { }
/// ```
#[proc_macro_attribute]
pub fn object(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	run(object::run(attr, item))
}
