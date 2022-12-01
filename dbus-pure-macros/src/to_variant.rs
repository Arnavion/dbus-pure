use super::ResultExt;

pub(super) fn run(input: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
	let input: proc_macro2::TokenStream = input.into();

	let tokens = input.clone();

	let input: syn::DeriveInput = syn::parse2(input)?;

	let ident = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let (signature_body, to_variant_body) = match input.data {
		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }), .. }) => {
			// Variant::Struct

			let fields_signature =
				fields.iter()
				.map(|syn::Field { ty, .. }| quote::quote! { <#ty as dbus_pure::proto::ToVariant>::signature() });

			let fields_to_variant =
				fields.iter()
				.map(|syn::Field { ident, .. }| quote::quote! { <_ as dbus_pure::proto::ToVariant>::to_variant(&self.#ident) });

			(
				quote::quote! {
					dbus_pure::proto::Signature::Struct {
						fields: vec![#(#fields_signature ,)*],
					}
				},
				quote::quote! {
					dbus_pure::proto::Variant::Struct {
						fields: vec![#(#fields_to_variant ,)*].into(),
					}
				},
			)
		},

		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: fields, .. }), .. }) if fields.len() == 1 => {
			// Delegate to the wrapped type's impl

			let syn::Field { ty, .. } = fields.into_iter().next().unwrap();

			(
				quote::quote! {
					<#ty as dbus_pure::proto::ToVariant>::signature()
				},
				quote::quote! {
					<dbus_pure::proto::ToVariant>::to_variant(&self.0)
				},
			)
		},

		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(..), .. }) =>
			return Err("#[derive(ToVariant)] cannot be used on tuple structs with one field").spanning(&tokens),

		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unit, .. }) =>
			return Err("#[derive(ToVariant)] cannot be used on unit structs").spanning(&tokens),

		syn::Data::Enum(_) | syn::Data::Union(_) =>
			return Err("#[derive(ToVariant)] can only be used on structs").spanning(&tokens),
	};

	let result = quote::quote! {
		impl #impl_generics dbus_pure::proto::ToVariant for #ident #ty_generics #where_clause {
			fn signature() -> dbus_pure::proto::Signature {
				#signature_body
			}

			fn to_variant<'__a>(&'__a self) -> dbus_pure::proto::Variant<'__a> {
				#to_variant_body
			}
		}
	};

	Ok(result)
}
