use super::ResultExt;

pub(super) struct AsVariant;

impl super::CustomDerive for AsVariant {
	fn run(input: syn::DeriveInput, tokens: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
		let ident = &input.ident;
		let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

		let (signature_body, as_variant_body) = match input.data {
			syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }), .. }) => {
				// Variant::Struct

				let fields_signature =
					fields.iter()
					.map(|syn::Field { ty, .. }| quote::quote! { <#ty as dbus_pure_proto::AsVariant>::signature() });

				let fields_as_variant =
					fields.iter()
					.map(|syn::Field { ident, .. }| quote::quote! { <_ as dbus_pure_proto::AsVariant>::as_variant(&self.#ident) });

				(
					quote::quote! {
						dbus_pure_proto::Signature::Struct {
							fields: vec![#(#fields_signature ,)*],
						}
					},
					quote::quote! {
						dbus_pure_proto::Variant::Struct {
							fields: vec![#(#fields_as_variant ,)*].into(),
						}
					},
				)
			},

			syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: fields, .. }), .. }) if fields.len() == 1 => {
				// Delegate to the wrapped type's impl

				let syn::Field { ty, .. } = fields.into_iter().next().unwrap();

				(
					quote::quote! {
						<#ty as dbus_pure_proto::AsVariant>::signature()
					},
					quote::quote! {
						<dbus_pure_proto::AsVariant>::as_variant(&self.0)
					},
				)
			},

			syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(..), .. }) =>
				return Err("#[derive(AsVariant)] cannot be used on tuple structs with one field").spanning(&tokens),

			syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unit, .. }) =>
				return Err("#[derive(AsVariant)] cannot be used on unit structs").spanning(&tokens),

			syn::Data::Enum(_) | syn::Data::Union(_) =>
				return Err("#[derive(AsVariant)] can only be used on structs").spanning(&tokens),
		};

		let result = quote::quote! {
			impl #impl_generics dbus_pure_proto::AsVariant for #ident #ty_generics #where_clause {
				fn signature() -> dbus_pure_proto::Signature {
					#signature_body
				}

				fn as_variant<'__a>(&'__a self) -> dbus_pure_proto::Variant<'__a> {
					#as_variant_body
				}
			}
		};

		Ok(result.into())
	}
}
