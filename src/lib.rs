use proc_macro::TokenStream;
use quote::quote;

use rule_input::RuleInput;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Bracket},
    LitStr, Token,
};

pub(crate) mod arg;
pub(crate) mod rule_input;
pub(crate) mod rule_param;
pub(crate) mod rule_param_field;
pub(crate) mod rule_param_type;

pub(crate) mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(param_type);
    syn::custom_keyword!(body);
    syn::custom_keyword!(output);
    syn::custom_keyword!(params);
    syn::custom_keyword!(label);

    // iRODS rule types
    syn::custom_keyword!(string);
}

#[proc_macro]
pub fn rule(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as RuleInput);

    TokenStream::new()
}
