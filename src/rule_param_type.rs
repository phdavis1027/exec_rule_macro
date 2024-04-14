use syn::parse::{Parse, ParseStream};

use crate::kw;

pub(crate) enum RuleParamType {
    String,
}

impl Parse for RuleParamType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::string) {
            input.parse::<kw::string>()?;
            Ok(RuleParamType::String)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected `string`",
            ))
        }
    }
}
