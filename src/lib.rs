use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{
    parse::{self, Parse},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Field, Fields, FieldsNamed, Ident, ItemStruct, LitStr, Result, Token, TypePath,
};

mod serialize;

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
Body literal must be valid rule language. Output literal must name a type exported from packe::exec_rule"#;

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
                .ok_or_else(|| syn::Error::new(input.span(), r#"expected 'output = <string literal>', which must name a type exported from packe::exec_rule"#))?,
        })
    }
}

struct Rule {
    // The struct being taught how to execute rules
    receiver: ItemStruct,

    // The output struct that will be deserialized
    // from the server's response
    output: TypePath,

    // The rule language that will be executed
    body: LitStr,
}

impl ToTokens for Rule {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        out.extend(self.receiver.clone().into_token_stream());
        out.extend(serialize::expand_serialization_impl(self).into_token_stream());
    }
}

impl Rule {
    fn try_new(args: Args, input: ItemStruct) -> syn::Result<Self> {
        let fields = match &input.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(syn::Error::new(
                    input.fields.span(),
                    "expected a struct with named fields",
                ))
            }
        };

        let output = Ident::new(args.output.value().as_str(), args.output.span());
        let output: TypePath = syn::parse(quote! { ::packe::exec_rule::#output }.into()).unwrap();

        Ok(Rule {
            receiver: input,
            output,
            body: args.body,
        })
    }
}

#[proc_macro_attribute]
pub fn rule(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match syn::parse::<Args>(args) {
        Ok(args) => args,
        Err(e) => return input_and_compile_error(input, e),
    };

    let input = match syn::parse::<ItemStruct>(input.clone()) {
        Ok(input) => input,
        Err(e) => return input_and_compile_error(input, e),
    };

    match Rule::try_new(args, input.clone()) {
        Ok(rule) => rule.into_token_stream().into(),
        Err(e) => input_and_compile_error(input.into_token_stream().into(), e),
    }
}

fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
