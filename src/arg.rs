use syn::{parse::Parse, punctuated::Punctuated, LitStr, Token};

use crate::{kw, rule_param::RuleParam};

pub(crate) enum Arg {
    Name {
        value: LitStr,
    },
    Output {
        value: LitStr,
    },
    Body {
        value: LitStr,
    },
    Params {
        value: Punctuated<RuleParam, Token![,]>,
    },
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::name) {
            input.parse::<kw::name>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<LitStr>()?;
            Ok(Arg::Name { value })
        } else if lookahead.peek(kw::output) {
            input.parse::<kw::output>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<LitStr>()?;
            Ok(Arg::Output { value })
        } else if lookahead.peek(kw::body) {
            input.parse::<kw::body>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<LitStr>()?;
            Ok(Arg::Body { value })
        } else if lookahead.peek(kw::params) {
            input.parse::<kw::params>()?;
            input.parse::<Token![:]>()?;
            let content;
            let _ = syn::bracketed!(content in input);
            let value = Punctuated::parse_terminated(&content)?;
            Ok(Arg::Params { value })
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected `name`, `output`, `body` or `params`",
            ))
        }
    }
}
