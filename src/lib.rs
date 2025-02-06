#![doc = include_str!("../README.md")]

mod prelude;
mod utils;

use prelude::*;

#[proc_macro]
pub fn infix(input: TokenStream) -> TokenStream {
    let infix = parse_macro_input!(input as Infix);

    let (lhs, fn_ident, rhs) = (infix.lhs, infix.fn_ident, infix.rhs);

    quote! {
        #fn_ident(#lhs, #rhs)
    }
    .into()
}
