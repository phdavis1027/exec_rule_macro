use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse::{self, Parse},
    spanned::Spanned,
    ItemStruct, Result, Token, TypePath,
};

mod arg;
mod rule;
mod serialize;
mod write_utils;

mod kw {
    syn::custom_keyword!(body);
    syn::custom_keyword!(output);
}

#[proc_macro_attribute]
pub fn rule(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match syn::parse::<arg::Args>(args) {
        Ok(args) => args,
        Err(e) => return input_and_compile_error(input, e),
    };

    let input = match syn::parse::<ItemStruct>(input.clone()) {
        Ok(input) => input,
        Err(e) => return input_and_compile_error(input, e),
    };

    match rule::Rule::try_new(args, input.clone()) {
        Ok(rule) => rule.into_token_stream().into(),
        Err(e) => {
            return input_and_compile_error(input.into_token_stream().into(), e);
        }
    }
}

fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
