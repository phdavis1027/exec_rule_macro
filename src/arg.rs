use syn::{parse::Parse, punctuated::Punctuated, LitStr, Token};

use crate::kw;

pub(crate) enum Arg {
    Body(LitStr),
    Name(LitStr),
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::body) {
            input.parse::<kw::body>()?;
            input.parse::<Token![=]>()?;
            let lit = input.parse::<LitStr>()?;
            Ok(Arg::Body(lit))
        } else if lookahead.peek(kw::output) {
            input.parse::<kw::output>()?;
            input.parse::<Token![=]>()?;
            let lit = input.parse::<LitStr>()?;
            Ok(Arg::Name(lit))
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) struct Args {
    pub(crate) body: LitStr,
    pub(crate) output: LitStr,
}

const ARGS_PUNCTUATED_PARSE_ERR: &str = r#"invalid rule definition, expected 'body = <string literal>, output = <string literal>'. 
Body literal must be valid rule language. Output literal must name a type exported from irods_client::exec_rule"#;

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<Arg, Token![,]>::parse_terminated(input).map_err(|mut err| {
            err.combine(syn::Error::new(err.span(), ARGS_PUNCTUATED_PARSE_ERR));

            err
        })?;

        let mut body = None;
        let mut output = None;

        for arg in args {
            match arg {
                Arg::Body(lit) => {
                    if body.is_some() {
                        return Err(syn::Error::new(lit.span(), "duplicate 'body' argument"));
                    }
                    body = Some(lit);
                }
                Arg::Name(lit) => {
                    if output.is_some() {
                        return Err(syn::Error::new(lit.span(), "duplicate 'output' argument"));
                    }
                    output = Some(lit);
                }
            }
        }

        Ok(Args {
            body: body.ok_or_else(|| syn::Error::new(input.span(), "expected 'body = <string literal>', which must be valid rule language"))?,
            output: output
                .ok_or_else(|| syn::Error::new(input.span(), r#"expected 'output = <string literal>', which must name a type exported from irods_client::exec_rule"#))?,
        })
    }
}
