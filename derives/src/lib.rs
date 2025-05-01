use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::{parse_macro_input, spanned::Spanned, Expr};

macro_rules! try_compile_error {
    ($x: expr) => {
        match $x {
            Ok(x) => x,
            Err(e) => return e.into_compile_error().into(),
        }
    };
}

mod promote;
mod vulkan_structure;

fn find_vkhandle_source(fields: &syn::Fields) -> (proc_macro2::TokenStream, &syn::Type) {
    #[inline]
    fn has_handle_attribute(f: &syn::Field) -> bool {
        f.attrs
            .iter()
            .any(|a| matches!(a.meta, syn::Meta::Path(ref path) if path.is_ident("handle")))
    }

    match fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => match unnamed.iter().position(has_handle_attribute)
        {
            Some(n) => (quote! { self.#n }, &unnamed[n].ty),
            None => (quote! { self.0 }, &unnamed[0].ty),
        },
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            let Some(f) = named.iter().find(|&f| has_handle_attribute(f)) else {
                panic!("Named fields struct must have one field that marked by #[handle]");
            };
            let n = f.ident.as_ref().unwrap();

            (quote! { self.#n }, &f.ty)
        }
        syn::Fields::Unit => panic!("Unit struct cannot auto-derive VkHandle"),
    }
}

fn find_parent_field(fields: &syn::Fields) -> (usize, &syn::Field) {
    #[inline]
    fn has_parent_attribute(f: &syn::Field) -> bool {
        f.attrs
            .iter()
            .any(|a| matches!(a.meta, syn::Meta::Path(ref path) if path.is_ident("parent")))
    }

    match fields {
        syn::Fields::Named(ref n) => {
            let mut parents = n.named.iter().enumerate().filter(|(_, n)| has_parent_attribute(n));

            let Some(p) = parents.next() else {
                panic!("could not find parent field");
            };
            if parents.next().is_some() {
                panic!("too many parent fields");
            }

            p
        }
        syn::Fields::Unnamed(ref u) => {
            let mut parents = u.unnamed.iter().enumerate().filter(|(_, n)| has_parent_attribute(n));

            let Some(p) = parents.next() else {
                panic!("could not find parent field");
            };
            if parents.next().is_some() {
                panic!("too many parent fields");
            }

            p
        }
        syn::Fields::Unit => panic!("unit structure has no parent field"),
    }
}

#[proc_macro_derive(VkHandle, attributes(handle))]
pub fn derive_handle(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let syn::Data::Struct(syn::DataStruct { ref fields, .. }) = input.data else {
        panic!("AutoDerive VkHandle can only be applied for structs");
    };
    let (handle_field_ref, handle_ty) = find_vkhandle_source(fields);

    quote! {
        impl #impl_generics crate::VkHandle for #name #ty_generics #where_clause {
            type Handle = #handle_ty;

            #[inline(always)]
            fn native_ptr(&self) -> Self::Handle {
                #handle_field_ref
            }
        }
        impl #impl_generics crate::VkHandleMut for #name #ty_generics #where_clause {
            #[inline(always)]
            fn native_ptr_mut(&mut self) -> Self::Handle {
                #handle_field_ref
            }
        }
    }
    .into()
}

struct VkObjectAttributes {
    span: Span,
    r#type: Option<Expr>,
}
impl VkObjectAttributes {
    #[inline]
    pub fn find_and_parse(input: &syn::DeriveInput) -> syn::Result<Option<Self>> {
        input
            .attrs
            .iter()
            .find(|a| a.path().is_ident("VkObject"))
            .map(Self::parse)
            .transpose()
    }

    pub fn parse(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut r#type = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("type") {
                r#type = Some(meta.value()?.parse::<Expr>()?);
                return Ok(());
            }

            Err(meta.error("unkown attribute"))
        })?;

        Ok(Self {
            span: attr.span(),
            r#type,
        })
    }
}

#[proc_macro_derive(VkObject, attributes(VkObject))]
pub fn derive_object(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let object_attr = match VkObjectAttributes::find_and_parse(&input) {
        Ok(Some(x)) => x,
        Ok(None) => {
            return syn::Error::new(Span::call_site(), "VkObject attribute required")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };
    let Some(object_type) = object_attr.r#type else {
        return syn::Error::new(object_attr.span, "No type specified")
            .into_compile_error()
            .into();
    };

    quote! {
        impl #impl_generics crate::VkObject for #name #ty_generics #where_clause {
            const TYPE: crate::vk::VkObjectType = #object_type;
        }
    }
    .into()
}

#[proc_macro_derive(InstanceChild, attributes(parent))]
pub fn derive_instance_child(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let syn::Data::Struct(ref s) = input.data else {
        return syn::Error::new(Span::call_site(), "No type except structure can derive InstanceChild")
            .into_compile_error()
            .into();
    };
    let (parent_index, parent_field) = find_parent_field(&s.fields);
    let parent_ty = &parent_field.ty;
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { &self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { &self.#x }
        }
    };

    quote! {
        impl #impl_generics crate::InstanceChild for #name #ty_generics #where_clause {
            type ConcreteInstance = #parent_ty;

            fn instance(&self) -> &Self::ConcreteInstance { #parent_field }
        }
    }
    .into()
}

#[proc_macro_derive(InstanceChildTransferrable, attributes(parent))]
pub fn derive_instance_child_transferrable(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let syn::Data::Struct(ref s) = input.data else {
        panic!("no type except structure can derive InstanceChild");
    };
    let (parent_index, parent_field) = find_parent_field(&s.fields);
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { self.#x }
        }
    };

    quote! {
        impl #impl_generics crate::InstanceChildTransferrable for #name #ty_generics #where_clause {
            fn transfer_instance(self) -> Self::ConcreteInstance { #parent_field }
        }
    }
    .into()
}

#[proc_macro_derive(DeviceChild, attributes(parent))]
pub fn derive_device_child(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let syn::Data::Struct(ref s) = input.data else {
        panic!("no type except structure can derive DeviceChild");
    };
    let (parent_index, parent_field) = find_parent_field(&s.fields);
    let parent_ty = &parent_field.ty;
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { &self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { &self.#x }
        }
    };

    quote! {
        impl #impl_generics crate::DeviceChild for #name #ty_generics #where_clause {
            type ConcreteDevice = #parent_ty;

            fn device(&self) -> &Self::ConcreteDevice { #parent_field }
        }
    }
    .into()
}

#[proc_macro_derive(DeviceChildTransferrable, attributes(parent))]
pub fn derive_device_child_transferrable(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let syn::Data::Struct(ref s) = input.data else {
        panic!("no type except structure can derive DeviceChild");
    };
    let (parent_index, parent_field) = find_parent_field(&s.fields);
    let parent_field = match parent_field.ident {
        Some(ref f) => quote! { self.#f },
        None => {
            let x = syn::Index::from(parent_index);
            quote! { self.#x }
        }
    };

    quote! {
        impl #impl_generics crate::DeviceChildTransferrable for #name #ty_generics #where_clause {
            fn transfer_device(self) -> Self::ConcreteDevice { #parent_field }
        }
    }
    .into()
}

#[proc_macro_derive(VulkanStructure, attributes(VulkanStructure))]
#[inline(always)]
pub fn derive_vulkan_structure(input: TokenStream) -> TokenStream {
    vulkan_structure::derive(input)
}

#[proc_macro_derive(VulkanSinkStructure, attributes(VulkanSinkStructure))]
#[inline(always)]
pub fn derive_vulkan_sink_structure(input: TokenStream) -> TokenStream {
    vulkan_structure::derive_sink(input)
}

#[proc_macro_attribute]
#[inline(always)]
pub fn promote_1_1(args: TokenStream, item: TokenStream) -> TokenStream {
    promote::core(
        parse_macro_input!(item as syn::Item),
        args,
        syn::LitStr::new("Allow1_1APIs", Span::call_site()),
    )
}
#[proc_macro_attribute]
#[inline(always)]
pub fn promote_1_2(args: TokenStream, item: TokenStream) -> TokenStream {
    promote::core(
        parse_macro_input!(item as syn::Item),
        args,
        syn::LitStr::new("Allow1_2APIs", Span::call_site()),
    )
}
#[proc_macro_attribute]
#[inline(always)]
pub fn promote_1_3(args: TokenStream, item: TokenStream) -> TokenStream {
    promote::core(
        parse_macro_input!(item as syn::Item),
        args,
        syn::LitStr::new("Allow1_3APIs", Span::call_site()),
    )
}
#[proc_macro_attribute]
#[inline(always)]
pub fn promote_1_4(args: TokenStream, item: TokenStream) -> TokenStream {
    promote::core(
        parse_macro_input!(item as syn::Item),
        args,
        syn::LitStr::new("Allow1_4APIs", Span::call_site()),
    )
}

#[inline]
fn newtype_struct_org_type(d: &syn::DataStruct) -> syn::Result<&syn::Type> {
    match d.fields {
        syn::Fields::Unnamed(ref f) if f.unnamed.len() == 1 => Ok(&unsafe { f.unnamed.first().unwrap_unchecked() }.ty),
        _ => Err(syn::Error::new_spanned(&d.fields, "not a newtype struct")),
    }
}

#[proc_macro_attribute]
pub fn vk_raw_handle(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let syn::Data::Struct(ref s) = input.data else {
        panic!("cannot derive VkRawHandle from this type");
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

    let dispatchable = matches!(try_compile_error!(newtype_struct_org_type(s)), syn::Type::Ptr(_));
    let (null_def, raw_handle_conversion);
    if dispatchable {
        null_def = quote! { Self(std::ptr::null_mut()) };
        raw_handle_conversion = quote! { self.0 as usize as _ };
    } else {
        null_def = quote! { Self(0) };
        raw_handle_conversion = quote! { self.0 };
    }

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

#[proc_macro_derive(PFN, attributes(pfn_of))]
pub fn derive_pfn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let impl_name = &input.ident;

    let Some(org_attr) = input.attrs.iter().find_map(|a| match a.meta {
        syn::Meta::List(ref ml) if ml.path.is_ident("pfn_of") => Some(ml),
        _ => None,
    }) else {
        return syn::Error::new_spanned(&input, "no #[pfn_of(...)] found")
            .into_compile_error()
            .into();
    };
    let org_fn: syn::Path = try_compile_error!(org_attr.parse_args());
    let Some(org_fn_name) = org_fn.get_ident().or_else(|| org_fn.segments.last().map(|l| &l.ident)) else {
        return syn::Error::new_spanned(org_attr, "invalid pfn_of fn path")
            .into_compile_error()
            .into();
    };
    let org_fn_cstr = syn::LitCStr::new(
        &unsafe { std::ffi::CString::from_vec_unchecked(org_fn_name.to_string().into_bytes()) },
        org_fn.span(),
    );

    quote! {
        unsafe impl crate::resolver::PFN for #impl_name {
            const NAME_CSTR: &'static core::ffi::CStr = #org_fn_cstr;

            #[inline(always)]
            unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self {
                core::mem::transmute(p)
            }
            #[inline(always)]
            unsafe fn from_void_fn(p: crate::vk::PFN_vkVoidFunction) -> Self {
                core::mem::transmute(p)
            }
        }
    }
    .into()
}

#[proc_macro_derive(StaticCallable)]
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
        impl #impl_generics crate::resolver::StaticCallable for #impl_name #ty_generics #where_clause {
            const STATIC: Self = Self(#org_fn);
        }
    }
    .into()
}

/// alias for `#[cfg(feature = "Implements")]`, with additional feature requirements
#[proc_macro_attribute]
pub fn implements(args: TokenStream, target: TokenStream) -> TokenStream {
    let mut out: TokenStream = quote! { #[cfg(feature = "Implements")] }.into();

    if !args.is_empty() {
        let extra_feature_requirements =
            parse_macro_input!(args with syn::punctuated::Punctuated<syn::LitStr, syn::Token![,]>::parse_terminated);

        out.extend(
            extra_feature_requirements
                .into_iter()
                .flat_map(|x| TokenStream::from(quote! { #[cfg(feature = #x)] })),
        );
    }

    out.extend(target);
    out
}

#[proc_macro_attribute]
pub fn bitflags_newtype(_args: TokenStream, target: TokenStream) -> TokenStream {
    let input = parse_macro_input!(target as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let syn::Data::Struct(ref s) = input.data else {
        return syn::Error::new(input.span(), "cannot use as bitflags struct")
            .into_compile_error()
            .into();
    };
    let org_type = try_compile_error!(newtype_struct_org_type(s));

    quote! {
        #[repr(transparent)]
        #input
        impl #impl_generics core::ops::BitOr for #name #ty_generics #where_clause {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
        impl #impl_generics core::ops::BitAnd for #name #ty_generics #where_clause {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }
        impl #impl_generics core::ops::Not for #name #ty_generics #where_clause {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
        impl #impl_generics core::ops::BitOrAssign for #name #ty_generics #where_clause {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl #impl_generics core::ops::BitAndAssign for #name #ty_generics #where_clause {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl #impl_generics From<#name #ty_generics> for #org_type #where_clause {
            #[inline(always)]
            fn from(value: #name #ty_generics) -> Self {
                value.0
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// Returns bits in this flags
            #[inline(always)]
            pub const fn bits(&self) -> #org_type {
                self.0
            }

            /// Returns true if any of specified bits are contained in this flag.
            #[inline(always)]
            pub const fn has_any(self, other: Self) -> bool {
                (self.0 & other.0) != 0
            }

            /// Returns true if all of specified bits are contained in this flag.
            #[inline(always)]
            pub const fn has_all(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// merge two flags (const alias of BitOr)
            #[inline(always)]
            pub const fn merge(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
    }
    .into()
}

struct VkExtCommandInput {
    base_define: syn::ForeignItemFn,
    suffix: syn::LitStr,
    promote: Option<syn::LitStr>,
    static_callable: bool,
}
impl syn::parse::Parse for VkExtCommandInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base_define = syn::ForeignItemFn::parse(input)?;

        let (mut suffix, mut promote, mut static_callable) = (None, None, false);
        while input.peek(syn::Ident) {
            let extra_ident = syn::Ident::parse(input)?;

            match &extra_ident.to_string() as &str {
                "suffix" => {
                    if suffix.is_some() {
                        return Err(syn::Error::new(input.span(), "duplicated suffix extra"));
                    }

                    input.parse::<syn::Token![=]>()?;
                    suffix = Some(input.parse()?);
                    input.parse::<syn::Token![;]>()?;
                }
                "promote" => {
                    if promote.is_some() {
                        return Err(syn::Error::new(input.span(), "duplicated promote extra"));
                    }

                    input.parse::<syn::Token![=]>()?;
                    promote = Some(input.parse()?);
                    input.parse::<syn::Token![;]>()?;
                }
                "static_callable" => {
                    input.parse::<syn::Token![;]>()?;

                    static_callable = true;
                }
                unknown => return Err(syn::Error::new(input.span(), &format!("unknown extra: {unknown}"))),
            }
        }

        Ok(Self {
            base_define,
            suffix: suffix.ok_or_else(|| syn::Error::new(input.span(), "suffix extra required"))?,
            promote,
            static_callable,
        })
    }
}

#[proc_macro]
pub fn vk_ext_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as VkExtCommandInput);

    let base_vis = &input.base_define.vis;
    let pfn_name = syn::Ident::new(
        &format!("PFN_{}", input.base_define.sig.ident.to_string()),
        input.base_define.sig.ident.span(),
    );
    let pfn_ty = syn::TypeBareFn {
        lifetimes: None,
        unsafety: Some(syn::Token![unsafe](Span::call_site())),
        abi: Some(syn::Abi {
            extern_token: syn::Token![extern](Span::call_site()),
            name: Some(syn::LitStr::new("system", Span::call_site())),
        }),
        fn_token: input.base_define.sig.fn_token.clone(),
        paren_token: input.base_define.sig.paren_token.clone(),
        inputs: input
            .base_define
            .sig
            .inputs
            .iter()
            .map(|a| match a {
                syn::FnArg::Receiver(_) => unreachable!("vk command cannot have receivers"),
                syn::FnArg::Typed(t) => syn::BareFnArg {
                    attrs: t.attrs.clone(),
                    name: match *t.pat {
                        syn::Pat::Ident(ref x) => Some((x.ident.clone(), t.colon_token.clone())),
                        _ => None,
                    },
                    ty: *t.ty.clone(),
                },
            })
            .collect(),
        variadic: input.base_define.sig.variadic.as_ref().map(|v| syn::BareVariadic {
            attrs: v.attrs.clone(),
            name: match v.pat {
                Some((ref p, c)) => match &**p {
                    &syn::Pat::Ident(ref x) => Some((x.ident.clone(), c.clone())),
                    _ => None,
                },
                _ => None,
            },
            dots: v.dots.clone(),
            comma: v.comma.clone(),
        }),
        output: input.base_define.sig.output.clone(),
    };
    let fname_cstr = syn::LitCStr::new(
        &unsafe { std::ffi::CString::from_vec_unchecked(input.base_define.sig.ident.to_string().into_bytes()) },
        input.base_define.sig.ident.span(),
    );

    let ext_static_link_impl = if input.static_callable {
        let fn_ident = &input.base_define.sig.ident;
        let define = &input.base_define;

        Some(quote! {
            #[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
            impl crate::resolver::StaticCallable for #pfn_name {
                const STATIC: Self = Self(#fn_ident);
            }

            #[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
            extern "system" {
                #define
            }
        })
    } else {
        None
    };

    let promoted_pfn_impl = if let Some(ref p) = input.promote {
        let base_fn_name = input.base_define.sig.ident.to_string();
        let Some(promoted_fn_name) = base_fn_name.strip_suffix(&input.suffix.value()) else {
            return syn::Error::new_spanned(&input.base_define.sig.ident, "not suffixed")
                .into_compile_error()
                .into();
        };
        let promoted_fn_ident = syn::Ident::new(promoted_fn_name, input.base_define.sig.ident.span());
        let pfn_name = syn::Ident::new(&format!("PFN_{promoted_fn_name}"), input.base_define.sig.ident.span());
        let promoted_fn_cstr = syn::LitCStr::new(
            &unsafe { std::ffi::CString::from_vec_unchecked(promoted_fn_name.as_bytes().to_vec()) },
            input.base_define.sig.ident.span(),
        );
        let promoted_fn = syn::ForeignItemFn {
            sig: syn::Signature {
                ident: promoted_fn_ident.clone(),
                ..input.base_define.sig.clone()
            },
            ..input.base_define.clone()
        };

        let promote_feature_name = syn::LitStr::new(&format!("Allow{}APIs", p.value().replace('.', "_")), p.span());

        Some(quote! {
            #[cfg(feature = #promote_feature_name)]
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #base_vis struct #pfn_name(pub #pfn_ty);
            #[cfg(feature = #promote_feature_name)]
            unsafe impl crate::resolver::PFN for #pfn_name {
                const NAME_CSTR: &'static core::ffi::CStr = #promoted_fn_cstr;

                #[inline(always)]
                unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self {
                    core::mem::transmute(p)
                }
                #[inline(always)]
                unsafe fn from_void_fn(p: crate::vk::PFN_vkVoidFunction) -> Self {
                    core::mem::transmute(p)
                }
            }

            #[cfg(all(feature = "Implements", feature = #promote_feature_name, not(feature = "DynamicLoaded")))]
            impl crate::resolver::StaticCallable for #pfn_name {
                const STATIC: Self = Self(#promoted_fn_ident);
            }

            #[cfg(all(feature = "Implements", feature = #promote_feature_name, not(feature = "DynamicLoaded")))]
            extern "system" {
                #promoted_fn
            }
        })
    } else {
        None
    };

    quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #base_vis struct #pfn_name(pub #pfn_ty);
        unsafe impl crate::resolver::PFN for #pfn_name {
            const NAME_CSTR: &'static core::ffi::CStr = #fname_cstr;

            #[inline(always)]
            unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self {
                core::mem::transmute(p)
            }
            #[inline(always)]
            unsafe fn from_void_fn(p: crate::vk::PFN_vkVoidFunction) -> Self {
                core::mem::transmute(p)
            }
        }

        #ext_static_link_impl
        #promoted_pfn_impl
    }
    .into()
}

/// Provides safe implementation for [`SpecializationConstants`] by deriving from structs.
#[proc_macro_derive(SpecializationConstants, attributes(constant_id))]
pub fn safe_derive_spec_constant(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let mut entries = Vec::with_capacity(fields.len());
            for f in fields {
                let mut constant_ids = f.attrs.iter().filter_map(|a| match a {
                    syn::Attribute {
                        style: syn::AttrStyle::Outer,
                        meta: syn::Meta::NameValue(ref nv),
                        ..
                    } if nv.path.is_ident("constant_id") => Some(Ok(nv.value.clone())),
                    syn::Attribute {
                        style: syn::AttrStyle::Outer,
                        meta: syn::Meta::List(ref ml),
                        ..
                    } if ml.path.is_ident("constant_id") => Some(ml.parse_args::<syn::Expr>()),
                    _ => None,
                });
                let Some(first_cid) = constant_ids.next() else {
                    return syn::Error::new_spanned(f, "Missing constant_id attribute")
                        .into_compile_error()
                        .into();
                };
                if constant_ids.next().is_some() {
                    return syn::Error::new_spanned(f, "One or more constant_id attributes found on same field")
                        .into_compile_error()
                        .into();
                }
                let constant_id = try_compile_error!(first_cid);

                let ty = &f.ty;
                let ident = &f.ident;
                entries.push(quote! { bedrock::SpecializationMapEntry {
                    constantID: #constant_id,
                    offset: core::mem::offset_of!(Self, #ident) as _,
                    size: core::mem::size_of::<#ty>(),
                } });
            }

            quote! {
                unsafe impl<#impl_generics> bedrock::SpecializationConstants for #name #ty_generics #where_clause {
                    const ENTRIES: &'static [bedrock::SpecializationMapEntry] = &[#(#entries),*];

                    #[inline(always)]
                    fn as_ptr(&self) -> *const core::ffi::c_void {
                        self as *const _ as _
                    }
                }
            }
            .into()
        }
        _ => unimplemented!("unsupported"),
    }
}
