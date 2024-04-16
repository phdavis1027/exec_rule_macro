use quote::quote;
use syn::{Ident, Type};

use crate::{
    exec_rule_out_types::OutputType, rule_input::RuleInput, rule_param::RuleParam,
    rule_param_type::RuleParamType,
};

pub fn expand_implementation(name: &Ident, input: &RuleInput) -> proc_macro2::TokenStream {
    let RuleInput { output, .. } = input;

    let output = match output {
        OutputType::ExecRuleOut => quote! { ::packe::exec_rule::ExecRuleOut },
    };

    let signature = input
        .params
        .iter()
        .map(|RuleParam { label, param_type }| {
            let rust_type = match param_type {
                RuleParamType::String => quote! { String },
                RuleParamType::Int32 => quote! { i32 },
                RuleParamType::Int16 => quote! { i16 },
                RuleParamType::Byte => quote! { u8 },
                RuleParamType::Double => quote! { f64 },
            };

            quote! {
                #label: #rust_type,
            }
        })
        .collect::<Vec<_>>();

    for it in signature.iter() {
        println!("{:?}", it);
    }

    quote! {}.into()
}
