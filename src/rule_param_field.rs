use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Token,
};

use crate::{kw, rule_param_type::RuleParamType};

pub(crate) enum RuleParamField {
    Label { value: Ident },
    ParamType { value: RuleParamType },
}

impl Parse for RuleParamField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::label) {
            input.parse::<kw::label>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<Ident>()?;
            Ok(RuleParamField::Label { value })
        } else if lookahead.peek(kw::param_type) {
            input.parse::<kw::param_type>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<RuleParamType>()?;
            Ok(RuleParamField::ParamType { value })
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected `label` or `param_type`",
            ))
        }
    }
}
