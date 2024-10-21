use darling::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

#[derive(FromDeriveInput)]
#[darling(supports(struct_newtype))]
pub struct TypeArgs {
    pub ident: Ident,
    pub generics: Generics,
    pub data: darling::ast::Data<(), Field>,
}

pub fn impl_type_deref(ast: DeriveInput) -> TokenStream2 {
    let ty_args = TypeArgs::from_derive_input(&ast).unwrap();
    let ty_ident = &ty_args.ident;
    let field_type = ty_args
        .data
        .take_struct()
        .unwrap()
        .fields
        .first()
        .unwrap()
        .ty
        .clone();

    let (impl_generics, ty_generics, where_clause) = ty_args.generics.split_for_impl();

    quote! {
        impl #impl_generics ::core::ops::Deref for #ty_ident #ty_generics #where_clause {
            type Target = #field_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}
