use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    quote! {
        impl crate::ecs::query::Component for #ident {
            type Inner = #ident;
            type RefInner<'w> = &'w mut #ident;

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
