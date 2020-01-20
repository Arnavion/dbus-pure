pub(super) fn run(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
	let Attr { interfaces } = syn::parse(attr)?;

	let input: proc_macro2::TokenStream = item.into();
	let input: syn::ItemStruct = syn::parse2(input)?;

	let vis = &input.vis;

	let struct_name = &input.ident;

	let impls =
		interfaces.iter()
		.map(|interface| quote::quote! {
			impl #interface for #struct_name<'_> { }
		});

	Ok(quote::quote! {
		#vis struct #struct_name<'a> {
			#vis name: std::borrow::Cow<'a, str>,
			#vis path: dbus_pure::proto::ObjectPath<'a>,
		}

		impl dbus_pure::proto::Object for #struct_name<'_> {
			fn name(&self) -> &str {
				&*self.name
			}

			fn path(&self) -> dbus_pure::proto::ObjectPath<'_> {
				dbus_pure::proto::ObjectPath((&*self.path.0).into())
			}
		}

		#(#impls)*
	})
}

struct Attr {
	interfaces: Vec<syn::Path>,
}

impl syn::parse::Parse for Attr {
	fn parse(input: syn::parse::ParseStream<'_>) -> Result<Self, syn::Error> {
		let interfaces: syn::punctuated::Punctuated<syn::Path, syn::Token![,]> =
			input.call(syn::punctuated::Punctuated::parse_terminated)?;
		let interfaces = interfaces.into_iter().collect();
		Ok(Attr {
			interfaces,
		})
	}
}
