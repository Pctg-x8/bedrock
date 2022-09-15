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
            }
        }
        _ => unimplemented!("Named Fields"),
    };

    TokenStream::from(implement)
}
