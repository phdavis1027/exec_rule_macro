use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token, Type,
};

use crate::{arg::Arg, exec_rule_out_types::OutputType, rule_param::RuleParam};

pub(crate) struct RuleInput {
    pub name: Ident,
    pub output: OutputType,
    pub body: LitStr,
    pub params: Punctuated<RuleParam, Token![,]>,
}

impl Parse for RuleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut output = None;
        let mut body = None;
        let mut params = None;

        let args = Punctuated::<Arg, Token![,]>::parse_terminated(input)?;

        for arg in args {
            match arg {
                Arg::Name { value } => {
                    if let Some(_) = name {
                        return Err(syn::Error::new_spanned(value, "Duplicate name argument"));
                    }
                    name = Some(value);
                }
                Arg::Output { value } => {
                    if let Some(_) = output {
                        return Err(syn::Error::new_spanned(
                            "output",
                            "Duplicate output argument",
                        ));
                    }
                    output = Some(value);
                }
                Arg::Body { value } => {
                    if let Some(_) = body {
                        return Err(syn::Error::new_spanned(value, "Duplicate body argument"));
                    }
                    body = Some(value);
                }
                Arg::Params { value } => {
                    if let Some(_) = params {
                        return Err(syn::Error::new_spanned(
                            "params",
                            "Duplicate params argument",
                        ));
                    }
                    params = Some(value);
                }
            }
        }

        Ok(RuleInput {
            name: name.unwrap(),
            output: output.unwrap(),
            body: body.unwrap(),
            params: params.unwrap(),
        })
    }
}
