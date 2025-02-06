use crate::prelude::*;

pub struct Infix {
    pub lhs: Expr,
    #[allow(dead_code, reason = "Parsing only")]
    pub sep_lhs: Token![#],
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn_ident: Ident,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[allow(dead_code, reason = "Parsing only")]
    pub sep_rhs: Token![#],
    pub rhs: Expr,
}

impl Parse for Infix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lhs: input.parse()?,
            sep_lhs: input.parse()?,
            fn_ident: input.parse()?,
            sep_rhs: input.parse()?,
            rhs: input.parse()?,
        })
    }
}
