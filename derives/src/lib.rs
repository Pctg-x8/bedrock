use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, Expr};

fn find_vkhandle_source(fields: &syn::Fields) -> Option<(proc_macro2::TokenStream, &syn::Type)> {
    // try finding vk_handle attributed field first
    let preferred = match fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => unnamed
            .iter()
            .position(|f| {
                f.attrs
                    .iter()
                    .any(|a| a.meta.require_path_only().is_ok_and(|ap| ap.is_ident("handle")))
            })
            .map(|x| (quote! { self.#x }, &unnamed[x].ty)),
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named
            .iter()
            .find(|f| {
                f.attrs
                    .iter()
                    .any(|a| a.meta.require_path_only().is_ok_and(|ap| ap.is_ident("handle")))
            })
            .map(|f| {
                let n = f.ident.as_ref().unwrap();
                (quote! { self.#n }, &f.ty)
            }),
        syn::Fields::Unit => None,
    };

    preferred.or_else(|| match fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => unnamed.first().map(|f| (quote! { self.0 }, &f.ty)),
        syn::Fields::Unit => panic!("Unit struct cannot auto-derive VkHandle"),
        syn::Fields::Named(_) => panic!("Named fields struct must has one field that marked by #[handle]"),
    })
}

#[proc_macro_derive(VkHandle, attributes(handle))]
pub fn derive_handle(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (handle_field_ref, handle_ty) = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        find_vkhandle_source(fields).expect("No suitable field representing handle source")
    } else {
        panic!("AutoDerive VkHandle can only be applied for structs");
    };

    let implement = quote! {
        impl #impl_generics crate::VkHandle for #name #ty_generics #where_clause {
            type Handle = #handle_ty;

            #[inline]
            fn native_ptr(&self) -> Self::Handle {
                #handle_field_ref
            }
        }
        impl #impl_generics crate::VkHandleMut for #name #ty_generics #where_clause {
            #[inline]
            fn native_ptr_mut(&mut self) -> Self::Handle {
                #handle_field_ref
            }
        }
    };

    implement.into()
}

#[proc_macro_derive(VkObject, attributes(VkObject))]
pub fn derive_object(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let object_attrs = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("VkObject"))
        .expect("VkObject attribute required");
    let mut object_type = None;
    object_attrs
        .parse_nested_meta(|meta| {
            if meta.path.is_ident("type") {
                object_type = Some(meta.value()?.parse::<Expr>()?);
                return Ok(());
            }

            Err(meta.error("unknown attrs"))
        })
        .expect("Failed to parse VkObject inner");
    let object_type = object_type.expect("No object type specified");
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics crate::VkObject for #name #ty_generics #where_clause {
            const TYPE: crate::vk::VkObjectType = #object_type;
        }
    })
}

fn find_parent_field(fields: &syn::Fields) -> (usize, &syn::Field) {
    match fields {
        syn::Fields::Named(ref n) => {
            let mut parents = n
                .named
                .iter()
                .enumerate()
                .filter(|(_, n)| {
                    n.attrs
                        .iter()
                        .any(|a| a.meta.require_path_only().map_or(false, |m| m.is_ident("parent")))
                })
                .collect::<Vec<_>>();
            match parents.len() {
                0 => panic!("could not find parent field"),
                1 => unsafe { parents.pop().unwrap_unchecked() },
                _ => panic!("too many parent fields"),
            }
        }
        syn::Fields::Unnamed(ref u) => {
            let mut parents = u
                .unnamed
                .iter()
                .enumerate()
                .filter(|(_, n)| {
                    n.attrs
                        .iter()
                        .any(|a| a.meta.require_path_only().map_or(false, |m| m.is_ident("parent")))
                })
                .collect::<Vec<_>>();
            match parents.len() {
                0 => panic!("could not find parent field"),
                1 => unsafe { parents.pop().unwrap_unchecked() },
                _ => panic!("too many parent fields"),
            }
        }
        syn::Fields::Unit => panic!("unit structure has no parent field"),
    }
}

#[proc_macro_derive(InstanceChild, attributes(parent))]
pub fn derive_instance_child(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (parent_index, parent_field) = match input.data {
        syn::Data::Struct(ref s) => find_parent_field(&s.fields),
        _ => panic!("no type except structure can derive InstanceChild"),
    };
    let parent_ty = &parent_field.ty;
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { &self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { &self.#x }
        }
    };

    TokenStream::from(quote! {
        impl #impl_generics crate::InstanceChild for #name #ty_generics #where_clause {
            type ConcreteInstance = #parent_ty;

            fn instance(&self) -> &Self::ConcreteInstance { #parent_field }
        }
    })
}

#[proc_macro_derive(InstanceChildTransferrable, attributes(parent))]
pub fn derive_instance_child_transferrable(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (parent_index, parent_field) = match input.data {
        syn::Data::Struct(ref s) => find_parent_field(&s.fields),
        _ => panic!("no type except structure can derive InstanceChild"),
    };
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { self.#x }
        }
    };

    TokenStream::from(quote! {
        impl #impl_generics crate::InstanceChildTransferrable for #name #ty_generics #where_clause {
            fn transfer_instance(self) -> Self::ConcreteInstance { #parent_field }
        }
    })
}

#[proc_macro_derive(DeviceChild, attributes(parent))]
pub fn derive_device_child(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (parent_index, parent_field) = match input.data {
        syn::Data::Struct(ref s) => find_parent_field(&s.fields),
        _ => panic!("no type except structure can derive DeviceChild"),
    };
    let parent_ty = &parent_field.ty;
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { &self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { &self.#x }
        }
    };

    TokenStream::from(quote! {
        impl #impl_generics crate::DeviceChild for #name #ty_generics #where_clause {
            type ConcreteDevice = #parent_ty;

            fn device(&self) -> &Self::ConcreteDevice { #parent_field }
        }
    })
}

#[proc_macro_derive(DeviceChildTransferrable, attributes(parent))]
pub fn derive_device_child_transferrable(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (parent_index, parent_field) = match input.data {
        syn::Data::Struct(ref s) => find_parent_field(&s.fields),
        _ => panic!("no type except structure can derive DeviceChild"),
    };
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { self.#x }
        }
    };

    TokenStream::from(quote! {
        impl #impl_generics crate::DeviceChildTransferrable for #name #ty_generics #where_clause {
            fn transfer_device(self) -> Self::ConcreteDevice { #parent_field }
        }
    })
}

#[proc_macro_derive(VulkanStructure, attributes(VulkanStructure))]
pub fn derive_vulkan_structure(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let attrs = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("VulkanStructure"))
        .expect("VulkanStructure required");
    let mut ty = None;
    attrs
        .parse_nested_meta(|meta| {
            if meta.path.is_ident("type") {
                ty = Some(meta.value()?.parse::<Expr>()?);
                return Ok(());
            }

            Err(meta.error("unknown attribute"))
        })
        .expect("Failed to parse VulkanStructure inner meta");
    let ty = ty.expect("No type specified");
    // TODO: some checks here......

    TokenStream::from(quote! {
        unsafe impl #impl_generics crate::VulkanStructure for #name #ty_generics #where_clause {
            const TYPE: VkStructureType = #ty;
        }
    })
}

fn promote_ext_ident(src: &syn::Ident) -> Option<syn::Ident> {
    src.to_string()
        .rsplit_once('_')
        .map(|(n, _)| syn::Ident::new(n, src.span()))
}

fn promote_suffixed_ident(src: &syn::Ident, suffix: &str) -> Option<syn::Ident> {
    Some(src.to_string())
        .as_deref()
        .and_then(|s| {
            if s.ends_with(&suffix) {
                Some(&s[..s.len() - suffix.len()])
            } else {
                None
            }
        })
        .map(|s| syn::Ident::new(s, src.span()))
}

#[proc_macro_attribute]
pub fn promote_1_1(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::Item);

    let mut suffix: Option<syn::LitStr> = None;
    if !args.is_empty() {
        let argparser = syn::meta::parser(|ctx| {
            if ctx.path.is_ident("suffix") {
                suffix = Some(ctx.value()?.parse()?);
                Ok(())
            } else {
                Err(ctx.error("unknown argument for promote_1_1"))
            }
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
            let promoted_ident = suffix.map_or_else(
                || promote_ext_ident(ident).expect("ident has no suffix?"),
                |s| promote_suffixed_ident(ident, &s.value()).expect("unexpected suffix"),
            );

            quote! {
                #item
                #[cfg(feature = "Allow1_1APIs")]
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
            let promoted_ident =
                promote_suffixed_ident(ident, &suffix.expect("suffix required").value()).expect("ident has no suffix?");

            quote! {
                #item
                #[cfg(feature = "Allow1_1APIs")]
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
            let promoted_ident =
                promote_suffixed_ident(ident, &suffix.expect("suffix required").value()).expect("ident has no suffix?");

            quote! {
                #item
                #[cfg(feature = "Allow1_1APIs")]
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
            let promoted_ident = promote_suffixed_ident(&sig.ident, &suffix.expect("suffix required").value())
                .expect("ident has no suffix?");
            let promoted_sig = syn::Signature {
                ident: promoted_ident,
                ..sig.clone()
            };

            quote! {
                #item
                #[cfg(feature = "Allow1_1APIs")]
                #(#attrs)* #vis #promoted_sig;
            }
            .into()
        }
        syn::Item::ForeignMod(_f) => unreachable!("foreign mod?"),
        syn::Item::Verbatim(v) => {
            let v2 = v.into();
            let syn::ForeignItemFn { attrs, vis, sig, .. } = parse_macro_input!(v2 as syn::ForeignItemFn);

            let promoted_ident = promote_suffixed_ident(&sig.ident, &suffix.expect("suffix required").value())
                .expect("ident has no suffix?");
            let promoted_sig = syn::Signature {
                ident: promoted_ident,
                ..sig.clone()
            };

            quote! {
                #(#attrs)* #vis #sig;
                #[cfg(feature = "Allow1_1APIs")]
                #(#attrs)* #vis #promoted_sig;
            }
            .into()
        }
        _ => unreachable!("unsupported item to promote 1.1"),
    }
}

fn newtype_struct_org_type(d: &syn::Data) -> &syn::Type {
    match d {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Unnamed(f) => match f.unnamed.first() {
                Some(fst) => {
                    if f.unnamed.len() > 1 {
                        panic!("tuple struct has more than one elements");
                    }

                    &fst.ty
                }
                None => panic!("unit struct?"),
            },
            syn::Fields::Named(_) => panic!("named struct is not allowed for deriving VkRawHandle"),
            syn::Fields::Unit => panic!("unit struct is not allowed for deriving VkRawHandle"),
        },
        _ => panic!("other than struct data cannot be derived VkRawHandle"),
    }
}

#[proc_macro_attribute]
pub fn vk_raw_handle(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let is_dispatchable = matches!(newtype_struct_org_type(&input.data), syn::Type::Ptr(_));
    let null_def = if is_dispatchable {
        quote! { Self(std::ptr::null_mut()) }
    } else {
        quote! { Self(0) }
    };

    let mut object_type = None::<Expr>;
    let parser = syn::meta::parser(|p| {
        if p.path.is_ident("object_type") {
            object_type = Some(p.value()?.parse()?);
            Ok(())
        } else {
            Err(p.error("unknown attr"))
        }
    });
    parse_macro_input!(args with parser);

    let raw_handle_conversion = if is_dispatchable {
        quote! { self.0 as usize as _ }
    } else {
        quote! { self.0 }
    };

    quote! {
        #input
        impl #impl_generics crate::handle::VkRawHandle for #name #ty_generics #where_clause {
            const OBJECT_TYPE: VkObjectType = #object_type;
            const NULL: Self = #null_def;

            #[inline]
            fn raw_handle_value(&self) -> u64 {
                #raw_handle_conversion
            }
        }
    }
    .into()
}
