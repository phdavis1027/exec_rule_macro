use std::fmt::format;

use proc_macro::TokenStream;

use quote::quote;
use rule_input::RuleInput;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Bracket},
    Ident, LitStr, Token,
};

use crate::rule_param_type::RuleParamType;

pub(crate) mod arg;
pub(crate) mod input_struct;
pub(crate) mod rule_input;
pub(crate) mod rule_param;
pub(crate) mod rule_param_field;
pub(crate) mod rule_param_type;
pub(crate) mod serialize;

pub(crate) mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(param_type);
    syn::custom_keyword!(body);
    syn::custom_keyword!(output);
    syn::custom_keyword!(params);
    syn::custom_keyword!(label);

    // iRODS rule types
    syn::custom_keyword!(string);
    syn::custom_keyword!(byte); // char is a keyword
    syn::custom_keyword!(int16);
    syn::custom_keyword!(int32);
    syn::custom_keyword!(double);
}

#[proc_macro]
pub fn rule(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as RuleInput);

    let input_struct_name = Ident::new(
        &format!("{}__Rule__Input", input.name.value()),
        input.name.span(),
    );

    let fields = input_struct::expand_fields(&input);

    let input_struct = input_struct::expand_input_struct(&input_struct_name, &fields);

    let serialize_impl = serialize::expand_serialize(&input, &input_struct_name);

    let out = quote! {
        #input_struct
        #serialize_impl
    };

    TokenStream::from(out)
}
