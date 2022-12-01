use super::ResultExt;

pub(super) fn run(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
	let interface_name: syn::Expr = syn::parse(attr)?;

	let input: proc_macro2::TokenStream = item.into();
	let input: syn::ItemTrait = syn::parse2(input)?;

	let vis = &input.vis;
	let struct_name = &input.ident;

	let mut impl_body = vec![];

	for item in &input.items {
		let (attrs, sig) = match item {
			syn::TraitItem::Method(syn::TraitItemMethod { attrs, sig, .. }) => (attrs, sig),

			impl_item => return Err("#[dbus_pure_macros::object] can only be applied to impl blocks that contain empty fn definitions").spanning(impl_item),
		};

		let dbus_fn_name_attr =
			attrs.iter()
			.next()
			.ok_or(r#"item is missing a `#[name = "..."]` attribute to set the D-Bus function name"#)
			.spanning(item)?;
		let dbus_fn_name_meta = dbus_fn_name_attr.parse_meta()?;
		let dbus_fn_name = match dbus_fn_name_meta {
			syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) if path.is_ident("name") => lit,
			meta => return Err(r#"unexpected attribute, expected `#[name = "..."]`"#).spanning(meta),
		};

		let fn_name = &sig.ident;

		let args = &sig.inputs;
		let args_variant =
			if args.is_empty() {
				quote::quote! { None }
			}
			else {
				let mut arg_variants = vec![];
				for arg in args {
					let (pat, ty) = match arg {
						syn::FnArg::Receiver(_) => return Err("fn cannot have a receiver parameter").spanning(arg),
						syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => (&**pat, &**ty)
					};
					let ident = match pat {
						syn::Pat::Ident(ident) => ident,
						_ => return Err("fn parameters can only be idents, not arbitrary patterns").spanning(arg),
					};
					let arg = match ty {
						syn::Type::Reference(_) => quote::quote!(#ident),
						_ => quote::quote!(&#ident),
					};
					arg_variants.push(arg);
				}

				quote::quote! {
					Some(&dbus_pure::proto::Variant::Tuple {
						elements: (&[
							#(<_ as dbus_pure::proto::ToVariant>::to_variant(#arg_variants),)*
						][..]).into(),
					})
				}
			};

		let (return_ty, return_expr) = match &sig.output {
			syn::ReturnType::Default => (
				quote::quote! { () },
				quote::quote! {
					let _ = body;
					Ok(())
				},
			),

			syn::ReturnType::Type(_, ty) => {
				let return_ty = quote::quote! { #ty };

				// If return type is `dbus_pure::proto::Variant`, return it as-is
				let is_variant = match &**ty {
					syn::Type::Path(syn::TypePath { path, .. }) => {
						let segments: Vec<_> =
							path.segments.iter()
							.take(3)
							.map(|path_segment| &path_segment.ident)
							.collect();
						let is_variant =
							segments.len() == 3 &&
							segments[0] == "dbus_pure" &&
							segments[1] == "proto" &&
							segments[2] == "Variant";
						is_variant
					},
					_ => false,
				};

				let return_expr =
					if is_variant {
						quote::quote! {
							let body =
								body
								.ok_or_else(|| dbus_pure::MethodCallError::UnexpectedResponse(None))?;
							Ok(body)
						}
					}
					else {
						quote::quote! {
							let body =
								body
								.ok_or_else(|| dbus_pure::MethodCallError::UnexpectedResponse(None))?;
							let body =
								serde::Deserialize::deserialize(body)
								.map_err(|err| dbus_pure::MethodCallError::UnexpectedResponse(Some(err)))?;
							Ok(body)
						}
					};

				(return_ty, return_expr)
			}
		};

		impl_body.push(quote::quote! {
			fn #fn_name(
				&self,
				client: &mut dbus_pure::Client,
				#args
			) -> std::result::Result<#return_ty, dbus_pure::MethodCallError> {
				let body =
					client.method_call(
						self.name(),
						self.path(),
						#interface_name,
						#dbus_fn_name,
						#args_variant,
					)?;
				#return_expr
			}
		});
	}

	Ok(quote::quote! {
		#vis trait #struct_name: dbus_pure::proto::Object {
			#(#impl_body)*
		}
	})
}
