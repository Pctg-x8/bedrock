use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;

fn promote_ext_ident(src: &syn::Ident) -> syn::Result<syn::Ident> {
    match src.to_string().rsplit_once('_') {
        Some((n, _)) => Ok(syn::Ident::new(n, src.span())),
        None => Err(syn::Error::new_spanned(src, "cannot determine suffix from this ident")),
    }
}

fn promote_suffixed_ident(src: &syn::Ident, suffix: &str) -> syn::Result<syn::Ident> {
    match src.to_string().strip_suffix(suffix) {
        Some(x) => Ok(syn::Ident::new(x, src.span())),
        None => Err(syn::Error::new_spanned(src, "unexpected_suffix")),
    }
}

pub fn core(item: syn::Item, args: TokenStream, feature_name: syn::LitStr) -> TokenStream {
    let mut suffix: Option<syn::LitStr> = None;
    if !args.is_empty() {
        let argparser = syn::meta::parser(|ctx| {
            if ctx.path.is_ident("suffix") {
                suffix = Some(ctx.value()?.parse()?);
                return Ok(());
            }

            Err(ctx.error("unknown argument for promote macro"))
        });

        parse_macro_input!(args with argparser);
    }

    match item {
        syn::Item::Const(syn::ItemConst {
            ref attrs,
            ref vis,
            ref ident,
            ref generics,
            ref ty,
            ..
        }) => {
            let promoted_ident = match suffix {
                Some(s) => try_compile_error!(promote_suffixed_ident(ident, &s.value())),
                None => try_compile_error!(promote_ext_ident(ident)),
            };

            quote! {
                #item
                #[cfg(feature = #feature_name)]
                #(#attrs)* #vis const #promoted_ident #generics: #ty = #ident;
            }
            .into()
        }
        syn::Item::Type(syn::ItemType {
            ref attrs,
            ref vis,
            ref ident,
            ref generics,
            ..
        }) => {
            let promoted_ident = match suffix {
                Some(s) => try_compile_error!(promote_suffixed_ident(ident, &s.value())),
                None => {
                    return syn::Error::new(Span::call_site(), "suffix must be specified")
                        .into_compile_error()
                        .into()
                }
            };

            quote! {
                #item
                #[cfg(feature = #feature_name)]
                #(#attrs)* #vis type #promoted_ident #generics = #ident;
            }
            .into()
        }
        syn::Item::Struct(syn::ItemStruct {
            ref vis,
            ref ident,
            ref generics,
            ..
        }) => {
            let promoted_ident = match suffix {
                Some(s) => try_compile_error!(promote_suffixed_ident(ident, &s.value())),
                None => {
                    return syn::Error::new(Span::call_site(), "suffix must be specified")
                        .into_compile_error()
                        .into()
                }
            };

            quote! {
                #item
                #[cfg(feature = #feature_name)]
                #vis type #promoted_ident #generics = #ident #generics;
            }
            .into()
        }
        syn::Item::Fn(syn::ItemFn {
            ref vis,
            ref attrs,
            ref sig,
            ..
        }) => {
            let promoted_ident = match suffix {
                Some(s) => try_compile_error!(promote_suffixed_ident(&sig.ident, &s.value())),
                None => {
                    return syn::Error::new(Span::call_site(), "suffix must be specified")
                        .into_compile_error()
                        .into()
                }
            };
            let promoted_sig = syn::Signature {
                ident: promoted_ident,
                ..sig.clone()
            };

            quote! {
                #item
                #[cfg(feature = #feature_name)]
                #(#attrs)* #vis #promoted_sig;
            }
            .into()
        }
        syn::Item::ForeignMod(_f) => unreachable!("foreign mod?"),
        syn::Item::Verbatim(v) => {
            let v2 = v.into();
            let syn::ForeignItemFn { attrs, vis, sig, .. } = parse_macro_input!(v2 as syn::ForeignItemFn);

            let promoted_ident = match suffix {
                Some(s) => try_compile_error!(promote_suffixed_ident(&sig.ident, &s.value())),
                None => {
                    return syn::Error::new(Span::call_site(), "suffix must be specified")
                        .into_compile_error()
                        .into()
                }
            };
            let promoted_sig = syn::Signature {
                ident: promoted_ident,
                ..sig.clone()
            };

            quote! {
                #(#attrs)* #vis #sig;
                #[cfg(feature = #feature_name)]
                #(#attrs)* #vis #promoted_sig;
            }
            .into()
        }
        _ => unreachable!("unsupported item to promote"),
    }
}
