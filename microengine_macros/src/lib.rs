use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GameObject)]
pub fn gameobject_macro_derive(input: TokenStream) -> TokenStream {
    let name = &syn::parse::<syn::DeriveInput>(input).unwrap().ident;
    let gen = quote! {
        impl GameObject for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
