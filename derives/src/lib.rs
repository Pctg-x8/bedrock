
use proc_macro::TokenStream;
use quote::*;

#[proc_macro_derive(VkHandle)]
pub fn derive_handle(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing Failed");

    let name = &input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data { fields } else {
        panic!("AutoDerive VkHandle can only be applied for structs");
    };
    let implement = match fields {
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let target_ty = &unnamed.first().expect("Empty Struct?").ty;

            quote! {
                impl crate::VkHandle for #name {
                    type Handle = #target_ty;
                    fn native_ptr(&self) -> Self::Handle { self.0 }
                }
            }
        },
        _ => unimplemented!("Named Fields")
    };

    TokenStream::from(implement)
}

#[proc_macro_derive(DeviceChild)]
pub fn derive_device_child(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing Failed");
    let name = &input.ident;

    TokenStream::from(quote!{
        impl crate::DeviceChild for #name {
            fn device(&self) -> &crate::Device { &self.1 }
        }
    })
}
