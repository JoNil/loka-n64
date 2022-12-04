use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SparseComponent)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    quote! {
        impl crate::ecs::component::Component for #ident {
            type Inner = #ident;
            type RefInner<'w> = &'w mut #ident;
            type Storage = crate::ecs::sparse_storage::SparseStorage<Self>;

            fn convert(v: &mut Self::Inner) -> Self::RefInner<'_> {
                v
            }

            fn empty<'w>() -> Self::RefInner<'w> {
                unreachable!()
            }
        }
    }
    .into()
}

#[proc_macro_derive(DenseComponent)]
pub fn derive_dense_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    quote! {
        impl crate::ecs::component::Component for #ident
        where
            #ident: Clone,
            #ident: Default,
        {
            type Inner = #ident;
            type RefInner<'w> = &'w mut #ident;
            type Storage = crate::ecs::dense_storage::DenseStorage<Self>;

            fn convert(v: &mut Self::Inner) -> Self::RefInner<'_> {
                v
            }

            fn empty<'w>() -> Self::RefInner<'w> {
                unreachable!()
            }
        }
    }
    .into()
}
