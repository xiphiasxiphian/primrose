#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)] // using panics in proc macros is standard

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

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

#[proc_macro_derive(WithBuilder)]
pub fn with_builder_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);

    let fields = if let Data::Struct(data_struct) = &data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("WithBuilder only supports structs with named fields");
        }
    } else {
        panic!("WithBuilder only supports structs");
    };

    let methods = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_ty = &f.ty;
        let method_name = format_ident!("with_{}", field_name);

        quote! {
            pub fn #method_name(mut self, #field_name: #field_ty) -> Self {
                self.#field_name = #field_name;
                self
            }
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#methods)*
        }
    };

    expanded.into()
}
