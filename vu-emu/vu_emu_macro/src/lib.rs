use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn emit_asm(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let name = item.sig.ident.clone();
    let vis = item.vis;
    let signature = item.sig;
    let mut rest = quote! {};

    for s in item.block.stmts {
        rest.extend(s.to_token_stream());
    }

    let format_str = format!("{} {{}},{{}},{{}}\n", name);

    quote! {
        #vis #signature {
            #[cfg(feature = "record_asm")]
            {
                self.asm += &format!(#format_str, vd, vs, vt);
            }

            #rest
        }
    }
    .into()
}
