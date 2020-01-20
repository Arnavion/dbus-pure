//! This crate contains custom derives related to the [`dbus-pure-proto`](https://crates.io/crates/dbus-pure-proto) crate.

mod as_variant;

trait CustomDerive: Sized {
	fn run(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error>;
}

fn run_custom_derive<T>(input: proc_macro::TokenStream) -> proc_macro::TokenStream where T: CustomDerive {
	let input: proc_macro2::TokenStream = input.into();
	let tokens = input.clone();
	let token_stream = match syn::parse2(input).and_then(|input| <T as CustomDerive>::run(input, tokens)) {
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
/// ```rust,ignore
/// #[derive(Debug, dbus_pure_macros::AsVariant, serde_derive::Deserialize)]
/// struct Response {
///     foo: u32,
///     bar: std::borrow::Cow<'a, str>,
/// }
/// ```
#[proc_macro_derive(AsVariant)]
pub fn derive_as_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	run_custom_derive::<as_variant::AsVariant>(input)
}
