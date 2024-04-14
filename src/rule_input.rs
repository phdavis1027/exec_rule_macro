use proc_macro::Punct;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};

use crate::{arg::Arg, rule_param::RuleParam};

pub(crate) struct RuleInput {
    pub name: LitStr,
    pub output: LitStr,
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
                    println!("Parsed name: {}", &value.value());
                    name = Some(value);
                }
                Arg::Output { value } => {
                    println!("Parsed output");
                    output = Some(value);
                }
                Arg::Body { value } => {
                    println!("Parsed body");
                    body = Some(value);
                }
                Arg::Params { value } => {
                    println!("Parsed params");
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
