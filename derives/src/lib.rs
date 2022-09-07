use proc_macro::TokenStream;
use quote::*;

#[proc_macro_derive(VkHandle, attributes(object_type))]
pub fn derive_handle(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing Failed");

    let object_type = input
        .attrs
        .iter()
        .find(|a| a.path.get_ident().map_or(false, |id| id == "object_type"))
        .expect("object_type attribute is required to auto-derive VkHandle");
    let object_type = match object_type.parse_meta() {
        Ok(syn::Meta::NameValue(nv)) => {
            if let syn::Lit::Str(ls) = nv.lit {
                syn::Ident::new(&ls.value(), proc_macro2::Span::call_site())
            } else {
                panic!("object_type needs String value")
            }
        }
        Ok(_) => unimplemented!("object_type = ???"),
        Err(e) => panic!("Attribute ParseError! {:?}", e),
    };

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
                    const TYPE: VkObjectType = #object_type;

                    #[inline]
                    fn native_ptr(&self) -> Self::Handle { self.0 }
                }
            }
        }
        _ => unimplemented!("Named Fields"),
    };

    TokenStream::from(implement)
}
