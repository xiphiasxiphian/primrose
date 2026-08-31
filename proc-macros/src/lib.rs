#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)] // using panics in proc macros is standard

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

/// Derive the Component trait
///
/// # Panics
/// Panics if the engine crate isn't avaliable to pull the trait from
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let found_crate = crate_name("engine").expect("engine must be present in Cargo.toml");

    let crate_base = match found_crate
    {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) =>
        {
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

/// Derive the Resource trait
///
/// # Panics
/// Panics if the engine crate isn't avaliable to pull the trait from
#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream
{
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let found_crate = crate_name("engine").expect("engine must be present in Cargo.toml");

    let crate_base = match found_crate
    {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) =>
        {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    };

    let crate_path = quote!(#crate_base::jade::ecs::resource);

    let output = quote! {
        impl #crate_path::Resource for #ident
        {
            // type Ref<'a> = &'a #ident;
            // type RefMut<'a> = &'a mut #ident;

            // fn id() -> std::any::TypeId { std::any::TypeId::of::<#ident>() }

            // fn resource_ref<'a>(&'a self) -> Self::Ref<'a> { self }
            // fn resource_ref_mut<'a>(&'a mut self) -> Self::RefMut<'a> { self }
        }
    };

    output.into()
}
