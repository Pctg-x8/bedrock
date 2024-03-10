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
                        .any(|a| a.meta.require_path_only().is_ok_and(|m| m.is_ident("parent")))
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
                        .any(|a| a.meta.require_path_only().is_ok_and(|m| m.is_ident("parent")))
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
    let input = parse_macro_input!(tok as syn::DeriveInput);
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
    let input = parse_macro_input!(tok as syn::DeriveInput);
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

struct NewtypePFNInput {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub _type_token: syn::Token![type],
    pub newtype_name: syn::Ident,
    pub _eq_token: syn::Token![=],
    pub org_type: syn::Type,
}
impl syn::parse::Parse for NewtypePFNInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: syn::Attribute::parse_outer(input)?,
            vis: input.parse()?,
            _type_token: input.parse()?,
            newtype_name: input.parse()?,
            _eq_token: input.parse()?,
            org_type: input.parse()?,
        })
    }
}
impl NewtypePFNInput {
    pub fn quote_base_define(&self) -> proc_macro2::TokenStream {
        let Self {
            attrs,
            vis,
            newtype_name,
            org_type,
            ..
        } = self;

        quote! { #(#attrs)* #[repr(transparent)] #vis struct #newtype_name(#org_type); }
    }

    pub fn quote_from_ptr_impl(&self) -> proc_macro2::TokenStream {
        let Self { newtype_name, .. } = self;

        quote! {
            unsafe impl crate::vkresolve::FromPtr for #newtype_name {
                unsafe fn from_ptr(p: *const libc::c_void) -> Self {
                    core::mem::transmute(p)
                }
            }
        }
    }

    pub fn quote_pfn_impl(&self) -> proc_macro2::TokenStream {
        let Self { newtype_name, .. } = self;
        let fname = self.original_function_name_nulbytes();

        quote! {
            unsafe impl crate::vkresolve::PFN for #newtype_name {
                const NAME_NUL: &'static [u8] = #fname;

                unsafe fn from_ptr(p: *const libc::c_void) -> Self {
                    core::mem::transmute(p)
                }
                unsafe fn from_void_fn(p: crate::vk::PFN_vkVoidFunction) -> Self {
                    core::mem::transmute(p)
                }
            }
        }
    }

    pub fn original_function_name_nulbytes(&self) -> syn::LitByteStr {
        let tyname_ident = match self.org_type {
            syn::Type::Path(ref p) => p
                .path
                .get_ident()
                .or_else(|| p.path.segments.last().map(|l| &l.ident))
                .expect("org tyname is not an ident?"),
            _ => panic!("unknown org type"),
        };
        let tyname_str = tyname_ident.to_string();
        let mut fname = tyname_str
            .strip_prefix("PFN_")
            .expect("TypeName must be prefixed by \"PFN_\"")
            .as_bytes()
            .to_vec();
        fname.push(0);

        syn::LitByteStr::new(&fname, tyname_ident.span())
    }
}
#[proc_macro]
pub fn newtype_pfn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NewtypePFNInput);

    let base_def = input.quote_base_define();
    let from_ptr_impl = input.quote_from_ptr_impl();
    let pfn_impl = input.quote_pfn_impl();

    quote! {
        #base_def
        #from_ptr_impl
        #pfn_impl
    }
    .into()
}

#[proc_macro_derive(PFN, attributes(pfn_of))]
pub fn derive_pfn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let impl_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let org_attr = &input
        .attrs
        .iter()
        .filter_map(|a| a.meta.require_list().ok())
        .find(|a| a.path.is_ident("pfn_of"))
        .expect("no #[pfn_of] found");
    let org_fn: syn::Path = org_attr.parse_args().expect("invalid arg for pfn_of");
    let org_fn_name = org_fn
        .get_ident()
        .or_else(|| org_fn.segments.last().map(|l| &l.ident))
        .expect("invalid pfn_of fn path");
    let mut org_fn_nulbytes = org_fn_name.to_string().into_bytes();
    org_fn_nulbytes.push(0);
    let org_fn_nulbytes = syn::LitByteStr::new(&org_fn_nulbytes, proc_macro2::Span::call_site().into());

    quote! {
        unsafe impl #impl_generics crate::vkresolve::PFN for #impl_name #ty_generics #where_clause {
            const NAME_NUL: &'static [u8] = #org_fn_nulbytes;

            unsafe fn from_ptr(p: *const libc::c_void) -> Self {
                core::mem::transmute(p)
            }
            unsafe fn from_void_fn(p: crate::vk::PFN_vkVoidFunction) -> Self {
                core::mem::transmute(p)
            }
        }
    }
    .into()
}

#[proc_macro_derive(StaticCallable, attributes(static_fn))]
pub fn derive_static_callable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let impl_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let org_attr = &input
        .attrs
        .iter()
        .filter_map(|a| a.meta.require_list().ok())
        .find(|a| a.path.is_ident("pfn_of"))
        .expect("no #[pfn_of] found");
    let org_fn: syn::Path = org_attr.parse_args().expect("invalid arg for pfn_of");

    quote! {
        #[cfg(all(not(feature = "DynamicLoaded"), feature = "Implements"))]
        impl #impl_generics crate::vkresolve::StaticCallable for #impl_name #ty_generics #where_clause {
            const STATIC: Self = Self(#org_fn);
        }
    }
    .into()
}

/// alias for `#[cfg(feature = "Implements")]`, with additional feature requirements
#[proc_macro_attribute]
pub fn implements(args: TokenStream, target: TokenStream) -> TokenStream {
    let args = if args.is_empty() {
        None
    } else {
        Some(parse_macro_input!(args with syn::punctuated::Punctuated<syn::LitStr, syn::Token![,]>::parse_terminated))
    };
    let t2 = proc_macro2::TokenStream::from(target);

    let extra_feature_requirements = args.map_or_else(Vec::new, |a| {
        a.into_iter().map(|x| quote! { #[cfg(feature = #x)] }).collect()
    });

    quote! { #[cfg(feature = "Implements")] #(#extra_feature_requirements)* #t2 }.into()
}

#[proc_macro_attribute]
pub fn bitflags_newtype(_args: TokenStream, target: TokenStream) -> TokenStream {
    let t2 = proc_macro2::TokenStream::from(target.clone());
    let input = parse_macro_input!(target as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        #t2
        impl #impl_generics core::ops::BitOr for #name #ty_generics #where_clause {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
        impl #impl_generics core::ops::BitAnd for #name #ty_generics #where_clause {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }
        impl #impl_generics core::ops::Not for #name #ty_generics #where_clause {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0)
            }
        }
        impl #impl_generics core::ops::BitOrAssign for #name #ty_generics #where_clause {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl #impl_generics core::ops::BitAndAssign for #name #ty_generics #where_clause {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// Returns true if specified bits are contained in this flag.
            #[inline(always)]
            pub const fn has(self, other: Self) -> bool {
                (self.0 & other.0) != 0
            }
        }
    }
    .into()
}
