use proc_macro_crate::{crate_name, FoundCrate};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let found_crate = crate_name("engine").expect("engine must be present in Cargo.toml");

    let crate_base = match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    };

    let crate_path = quote!(#crate_base::jade::ecs::components);

    let output = quote! {
        impl #crate_path::Component for #ident {}
    };

    output.into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let found_crate = crate_name("engine").expect("engine must be present in Cargo.toml");

    let crate_base = match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    };

    let crate_path = quote!(#crate_base::jade::ecs::world);

    let output = quote! {
        impl #crate_path::Resource for #ident {}
    };

    output.into()
}
