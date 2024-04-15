use syn::parse::{Parse, ParseStream};

use crate::kw;

pub(crate) enum RuleParamType {
    String,
    Byte,
    Int16,
    Int32,
    Double,
}

impl Parse for RuleParamType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::string) {
            input.parse::<kw::string>()?;
            Ok(RuleParamType::String)
        } else if lookahead.peek(kw::byte) {
            input.parse::<kw::byte>()?;
            Ok(RuleParamType::Byte)
        } else if lookahead.peek(kw::int16) {
            input.parse::<kw::int16>()?;
            Ok(RuleParamType::Int16)
        } else if lookahead.peek(kw::int32) {
            input.parse::<kw::int32>()?;
            Ok(RuleParamType::Int32)
        } else if lookahead.peek(kw::double) {
            input.parse::<kw::double>()?;
            Ok(RuleParamType::Double)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected `string`",
            ))
        }
    }
}
