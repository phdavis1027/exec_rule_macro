use crate::{rule_param_field::RuleParamField, rule_param_type::RuleParamType};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    LitStr, Token,
};

pub(crate) struct RuleParam {
    pub label: LitStr,
    pub param_type: RuleParamType,
}

impl Parse for RuleParam {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        let input;
        let _: Brace = syn::braced!(input in _input);

        let mut label: Option<_> = None;
        let mut param_type: Option<_> = None;

        let fields = Punctuated::<RuleParamField, Token![,]>::parse_terminated(&input)?;

        for field in fields {
            match field {
                RuleParamField::Label { value } => {
                    if let Some(_) = label {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            "Duplicate `label` field",
                        ));
                    } else {
                        label = Some(value);
                    }
                }
                RuleParamField::ParamType { value } => {
                    if let Some(_) = param_type {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            "Duplicate `param_type` field",
                        ));
                    } else {
                        param_type = Some(value);
                    }
                }
            }
        }

        Ok(RuleParam {
            label: label.unwrap(),
            param_type: param_type.unwrap(),
        })
    }
}
