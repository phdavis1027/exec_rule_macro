use darling::FromDeriveInput;
use proc_macro::{Ident, TokenStream};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Token, Attribute, DataStruct,
    ItemImpl, ItemStruct, LitStr, Result, Token,
};

mod kw {
    syn::custom_keyword!(body);
    syn::custom_keyword!(output);
}

enum Arg {
    Body(LitStr),
    Name(LitStr),
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
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

struct Args {
    body: LitStr,
    output: LitStr,
}

const ARGS_PUNCTUATED_PARSE_ERR: &str = r#"invalid rule definition, expected 'body = <string literal>, output = <string literal>'. 
Body literal must be valid rule language. Output literal must name a type that implement `packe::bosd::Deserializable`"#;

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
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
                .ok_or_else(|| syn::Error::new(input.span(), r#"expected 'output = <string literal>', which must name a type that implements packe::bosd::Deserializable"#))?,
        })
    }
}

fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}

struct Rule {
    serializable_struct: ItemStruct,
    serialization_impl: ItemImpl,
    rule_execution_impl: ItemImpl,
}

#[proc_macro_attribute]
pub fn rule(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match syn::parse::<Args>(args) {
        Ok(args) => TokenStream::new(),
        Err(e) => return input_and_compile_error(input, e),
    };

    let input = match syn::parse::<ItemStruct>(input.clone()) {
        Ok(input) => input,
        Err(e) => return input_and_compile_error(input, e),
    };
}
