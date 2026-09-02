#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)] // using panics in proc macros is standard

use darling::{FromDeriveInput, FromField, ast, util::Ignored};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, Generics, Ident, PathArguments, Type, parse_macro_input};

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
        FoundCrate::Itself =>
        {
            let compiling_crate_name = std::env::var("CARGO_CRATE_NAME").unwrap_or_default();

            if compiling_crate_name == "engine"
            {
                quote!(crate)
            }
            else
            {
                quote!(::engine)
            }
        }
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

fn extract_inner_type(ty: &syn::Type) -> Option<&syn::Type>
{
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return Some(inner_ty);
    }

    None
}

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
struct BuilderField
{
    ident: Option<Ident>,
    ty: Type,
    // #[builder(skip)]
    #[darling(default)]
    skip: bool,
    // #[builder(into_some)]
    #[darling(default)]
    into_some: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderStruct
{
    ident: Ident,
    generics: Generics,
    data: ast::Data<Ignored, BuilderField>,
}

#[proc_macro_derive(WithBuilder, attributes(builder))]
pub fn with_builder_derive(input: TokenStream) -> TokenStream
{
    let ast = parse_macro_input!(input as DeriveInput);
    let BuilderStruct { ident, generics, data } = match BuilderStruct::from_derive_input(&ast)
    {
        Ok(parsed) => parsed,
        Err(e) => return e.write_errors().into(),
    };

    let struct_name = &ident;
    let fields = data.take_struct().unwrap();

    let methods = fields.into_iter().filter(|f| !f.skip).map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let method_name = format_ident!("with_{}", field_name);
        let field_ty = &f.ty;

        if f.into_some
        {
            if let Some(inner_ty) = extract_inner_type(field_ty)
            {
                quote! {
                    pub fn #method_name(mut self, #field_name: #inner_ty) -> Self {
                        self.#field_name = ::core::option::Option::Some(#field_name);
                        self
                    }
                }
            }
            else
            {
                syn::Error::new_spanned(field_ty, "`into_some` can only be used on Option<T> fields").to_compile_error()
            }
        }
        else
        {
            quote! {
                pub fn #method_name(mut self, #field_name: #field_ty) -> Self {
                    self.#field_name = #field_name;
                    self
                }
            }
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#methods)*
        }
    };

    expanded.into()
}
