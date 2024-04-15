use crate::{
    rule_param_field::RuleParamField,
    rule_param_type::RuleParamType,
    serialize::{write_end, write_start, write_tag, write_tag_fmt},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Ident, LitStr, Token,
};

pub(crate) struct RuleParam {
    pub label: LitStr,
    pub param_type: RuleParamType,
}

fn write_string_value(label: &LitStr) -> TokenStream {
    let start = write_start("STR_PI");
    let end = write_end("STR_PI");

    let field = Ident::new(label.value().as_str(), label.span());

    let value = write_tag(
        "myStr",
        quote! {
            self.#field.as_str()
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

fn write_byte_value(label: &LitStr) -> TokenStream {
    let start = write_start("CHAR_PI");
    let end = write_end("CHAR_PI");

    let field = Ident::new(label.value().as_str(), label.span());

    let value = write_tag_fmt(
        "myChar",
        LitStr::new("{}", label.span()),
        quote! {
            self.#field
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

fn write_int16_value(label: &LitStr) -> TokenStream {
    let start = write_start("INT16_PI");
    let end = write_end("INT16_PI");

    let field = Ident::new(label.value().as_str(), label.span());

    let value = write_tag_fmt(
        "myInt",
        LitStr::new("{}", label.span()),
        quote! {
            self.#field
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

fn write_int32_value(label: &LitStr) -> TokenStream {
    let start = write_start("INT_PI");
    let end = write_end("INT_PI");

    let field = Ident::new(label.value().as_str(), label.span());

    let value = write_tag_fmt(
        "myInt",
        LitStr::new("{}", label.span()),
        quote! {
            self.#field
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

fn write_double_value(label: &LitStr) -> TokenStream {
    let start = write_start("DOUBLE_PI");
    let end = write_end("DOUBLE_PI");

    let field = Ident::new(label.value().as_str(), label.span());

    let value = write_tag_fmt(
        "myDouble",
        LitStr::new("{}", label.span()),
        quote! {
            self.#field
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

impl RuleParam {
    pub fn tokify(param: &Self) -> TokenStream {
        let start = write_start("MsParam_PI");
        let end = write_end("MsParam_PI");

        let label = write_tag(
            "label",
            LitStr::new(
                format!("*{}", param.label.value()).as_str(),
                param.label.span(),
            ),
        );

        let param_type = match param.param_type {
            RuleParamType::String => write_tag("type", LitStr::new("STR_PI", param.label.span())),
            RuleParamType::Byte => write_tag("type", LitStr::new("CHAR_PI", param.label.span())),
            RuleParamType::Int16 => write_tag("type", LitStr::new("INT16_PI", param.label.span())),
            RuleParamType::Int32 => write_tag("type", LitStr::new("INT_PI", param.label.span())),
            RuleParamType::Double => {
                write_tag("type", LitStr::new("DOUBLE_PI", param.label.span()))
            }
        };

        let value = match param.param_type {
            RuleParamType::String => write_string_value(&param.label),
            RuleParamType::Byte => write_byte_value(&param.label),
            RuleParamType::Int16 => write_int16_value(&param.label),
            RuleParamType::Int32 => write_int32_value(&param.label),
            RuleParamType::Double => write_double_value(&param.label),
        };

        quote! {
            #start

            #label
            #param_type
            #value

            #end
        }
    }
}

impl Parse for RuleParam {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        let input;
        let _: Brace = syn::braced!(input in _input);

        let mut label: Option<_> = None;
        let mut param_type: Option<_> = None;

        let fields = Punctuated::<RuleParamField, Token![,]>::parse_terminated(&input)?;

        for field in fields {
            match field {
                RuleParamField::Label { value } => {
                    if let Some(_) = label {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            "Duplicate `label` field",
                        ));
                    } else {
                        label = Some(value);
                    }
                }
                RuleParamField::ParamType { value } => {
                    if let Some(_) = param_type {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            "Duplicate `param_type` field",
                        ));
                    } else {
                        param_type = Some(value);
                    }
                }
            }
        }

        Ok(RuleParam {
            label: label.unwrap(),
            param_type: param_type.unwrap(),
        })
    }
}
