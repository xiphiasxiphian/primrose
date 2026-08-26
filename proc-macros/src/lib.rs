use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Component for #ident {}
    };

    output.into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Resource for #ident {}
    };

    output.into()
}
