
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

#[proc_macro_derive(DeviceChild, attributes(drop_function_name))]
pub fn derive_device_child(tok: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(tok).expect("Parsing Failed");
    let name = &input.ident;
    let drop_function_name_attr = input.attrs.iter()
        .find(|a| a.path.get_ident().map_or(false, |id| id == "drop_function_name"));
    
    let drop_impl = drop_function_name_attr.map(|a| {
        let fname = match a.parse_meta() {
            Ok(syn::Meta::NameValue(nv)) => if let syn::Lit::Str(ls) = nv.lit {
                syn::Ident::new(&ls.value(), proc_macro2::Span::call_site())
            } else {
                panic!("drop_function_name needs String value")
            },
            Ok(_) => unimplemented!("drop_function_name = ???"),
            Err(e) => panic!("Attribute ParseError! {:?}", e)
        };

        quote!{
            #[cfg(feature = "Implements")]
            impl Drop for #name {
                fn drop(&mut self) {
                    unsafe {
                        crate::Resolver::get().#fname(self.1.native_ptr(), self.0, std::ptr::null());
                    }
                }
            }
        }
    });

    TokenStream::from(quote!{
        impl crate::DeviceChild for #name {
            fn device(&self) -> &crate::Device { &self.1 }
        }
        #drop_impl
    })
}