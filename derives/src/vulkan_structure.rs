use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Expr, Field, Fields, FieldsNamed};

pub struct VulkanStructureAttribute {
    pub span: Span,
    pub r#type: Option<Expr>,
}
impl VulkanStructureAttribute {
    fn parse(a: &syn::Attribute) -> syn::Result<Self> {
        let mut r#type = None;

        a.parse_nested_meta(|meta| {
            if meta.path.is_ident("type") {
                r#type = Some(meta.value()?.parse::<Expr>()?);
                return Ok(());
            }

            Err(meta.error("unknown attribute"))
        })?;

        Ok(Self { span: a.span(), r#type })
    }

    #[inline]
    pub fn find_and_parse(input: &syn::DeriveInput) -> syn::Result<Option<Self>> {
        input
            .attrs
            .iter()
            .find(|a| a.path().is_ident("VulkanStructure"))
            .map(Self::parse)
            .transpose()
    }

    #[inline]
    pub fn find_sink_and_parse(input: &syn::DeriveInput) -> syn::Result<Option<Self>> {
        input
            .attrs
            .iter()
            .find(|a| a.path().is_ident("VulkanSinkStructure"))
            .map(Self::parse)
            .transpose()
    }
}

#[inline]
pub fn require_named_field_struct(input: &DeriveInput) -> syn::Result<&FieldsNamed> {
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref x),
            ..
        }) => Ok(x),
        _ => Err(syn::Error::new_spanned(
            input,
            "VulkanStructure can be derived only from named-field structs",
        )),
    }
}

pub fn extract_required_head_fields(input: &FieldsNamed) -> syn::Result<(&Field, &Field)> {
    let (Some(first_field), Some(second_field)) = (input.named.first(), input.named.get(1)) else {
        return Err(syn::Error::new(
            input.span(),
            "VulkanStructure requires at least 2 fields in the struct",
        ));
    };

    if !first_field.ident.as_ref().is_some_and(|x| x == "sType") {
        return Err(syn::Error::new(
            first_field.span(),
            "VulkanStructure requires `sType` field at first position",
        ));
    }

    if !second_field.ident.as_ref().is_some_and(|x| x == "pNext") {
        return Err(syn::Error::new(
            second_field.span(),
            "VulkanStructure requires `pNext` field at second position",
        ));
    }

    Ok((first_field, second_field))
}

pub fn derive(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = match VulkanStructureAttribute::find_and_parse(&input) {
        Ok(Some(x)) => x,
        Ok(None) => {
            return syn::Error::new(Span::call_site(), "VulkanStructure attribute required")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };
    let Some(ty) = attrs.r#type else {
        return syn::Error::new(attrs.span, "No type specified")
            .into_compile_error()
            .into();
    };

    let fields = try_compile_error!(require_named_field_struct(&input));
    try_compile_error!(extract_required_head_fields(&fields));

    quote! {
        unsafe impl #impl_generics crate::VulkanStructureAsRef for #name #ty_generics #where_clause {
            #[inline(always)]
            fn as_generic(&self) -> &crate::GenericVulkanStructure {
                unsafe { core::mem::transmute(self) }
            }

            #[inline(always)]
            fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
                unsafe { core::mem::transmute(self) }
            }
        }
        unsafe impl #impl_generics crate::VulkanStructure for #name #ty_generics #where_clause {
            const TYPE: VkStructureType = #ty;
        }
    }
    .into()
}

pub fn derive_sink(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = match VulkanStructureAttribute::find_sink_and_parse(&input) {
        Ok(Some(x)) => x,
        Ok(None) => {
            return syn::Error::new(Span::call_site(), "VulkanSinkStructure attribute required")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };
    let Some(ty) = attrs.r#type else {
        return syn::Error::new(attrs.span, "No type specified")
            .into_compile_error()
            .into();
    };

    let fields = try_compile_error!(require_named_field_struct(&input));
    let (_, next_ptr_field) = try_compile_error!(extract_required_head_fields(&fields));
    if !matches!(
        next_ptr_field.ty,
        syn::Type::Ptr(syn::TypePtr {
            mutability: Some(_),
            const_token: None,
            ..
        })
    ) {
        return syn::Error::new(next_ptr_field.ty.span(), "`pNext` field must be a mutable pointer type")
            .into_compile_error()
            .into();
    }

    quote! {
        unsafe impl #impl_generics crate::VulkanSinkStructureAsRef for #name #ty_generics #where_clause {
            #[inline(always)]
            fn as_generic(&self) -> &crate::GenericVulkanSinkStructure {
                unsafe { core::mem::transmute(self) }
            }

            #[inline(always)]
            fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure {
                unsafe { core::mem::transmute(self) }
            }
        }
        unsafe impl #impl_generics crate::VulkanSinkStructure for #name #ty_generics #where_clause {
            const TYPE: VkStructureType = #ty;
        }
    }
    .into()
}
