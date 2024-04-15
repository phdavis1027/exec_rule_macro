use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{rule_input::RuleInput, rule_param_type::RuleParamType};

pub(crate) fn expand_input_struct(name: &Ident, params: &Vec<TokenStream>) -> TokenStream {
    quote! {
        #[derive(Debug)]
        pub struct #name {
            #(#params)*
            pub addr: ::std::net::SocketAddr,
            pub rods_zone: String,
            pub rule_engine_instance: Option<String>
        }
    }
}

pub(crate) fn expand_fields(input: &RuleInput) -> Vec<TokenStream> {
    input
        .params
        .iter()
        .map(|param| {
            let name = Ident::new(&param.label.value(), param.label.span());
            let ptype = match &param.param_type {
                RuleParamType::String => quote! { String },
                RuleParamType::Byte => quote! { u8 },
                RuleParamType::Int32 => quote! { i32 },
                RuleParamType::Int16 => quote! { i16 },
                RuleParamType::Double => quote! { f64 },
            };

            quote! {
                pub #name: #ptype,
            }
        })
        .collect()
}
