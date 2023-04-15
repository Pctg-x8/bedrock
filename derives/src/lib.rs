use proc_macro::TokenStream;
use quote::*;

#[proc_macro_derive(VkHandle)]
pub fn derive_handle(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing Failed");

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        fields
    } else {
        panic!("AutoDerive VkHandle can only be applied for structs");
    };
    let implement = match fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let target_ty = &unnamed.first().expect("Empty Struct?").ty;

            quote! {
                impl #impl_generics crate::VkHandle for #name #ty_generics #where_clause {
                    type Handle = #target_ty;

                    #[inline]
                    fn native_ptr(&self) -> Self::Handle { self.0 }
                }
                impl #impl_generics crate::VkHandleMut for #name #ty_generics #where_clause {
                    #[inline]
                    fn native_ptr_mut(&mut self) -> Self::Handle { self.0 }
                }
            }
        }
        _ => unimplemented!("Named Fields"),
    };

    TokenStream::from(implement)
}

#[proc_macro_derive(VkObject, attributes(object_type))]
pub fn derive_object(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let object_type_attr = input
        .attrs
        .iter()
        .find(|a| a.meta.path().is_ident("object_type"))
        .expect("object_type required");
    let object_type = &object_type_attr.meta.require_name_value().expect("object_type requires name-value form").value;
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
                    n.attrs.iter().any(|a| a.meta.require_path_only().map_or(false, |m| m.is_ident("parent")))
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
                    n.attrs.iter().any(|a| a.meta.require_path_only().map_or(false, |m| m.is_ident("parent")))
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

#[proc_macro_derive(VulkanStructure, attributes(structure_type))]
pub fn derive_vulkan_structure(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing failed");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ty_attr = input.attrs.iter().find(|a| a.path().is_ident("structure_type")).expect("structure_type required");
    let ty = &ty_attr.meta.require_name_value().expect("expected #[structure_type = \"...\"]").value;
    // TODO: some checks here......

    TokenStream::from(quote! {
        unsafe impl #impl_generics crate::VulkanStructure for #name #ty_generics #where_clause {
            const TYPE: VkStructureType = #ty;
        }
    })
}
