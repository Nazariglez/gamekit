extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_state_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics GKState for #name #ty_generics #where_clause {}
        impl #impl_generics FromStorage<#name #ty_generics> for #name #ty_generics #where_clause {
            fn from_storage(storage: &mut Storage<#name #ty_generics>) -> &mut Self {
                &mut storage.state
            }
        }
    };
    gen.into()
}
