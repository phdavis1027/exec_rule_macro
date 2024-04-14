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

    let input_struct_name = Ident::new(
        &format!("{}__Rule__Input", input.name.value()),
        input.name.span(),
    );

    let input_struct_params = input
        .params
        .iter()
        .map(|param| {
            let param_name = Ident::new(&param.label.value(), param.label.span());
            let param_type = match &param.param_type {
                RuleParamType::String => quote! { String },
            };
            quote! {
                pub #param_name: #param_type,
            }
        })
        .collect::<Vec<_>>();

    let input_struct_param_labels = input
        .params
        .iter()
        .map(|param| {
            let param_name = Ident::new(&param.label.value(), param.label.span());
            quote! {
                #param_name,
            }
        })
        .collect::<Vec<_>>();

    let rule_body = LitStr::new(
        format!(
            r##"
    @external
    {} {{
        {}        
    }}
    "##,
            input.name.value(),
            input.body.value()
        )
        .as_str(),
        input.body.span(),
    );

    let out = quote! {
        #[derive(Debug)]
        pub struct #input_struct_name {
            #(#input_struct_params)*
            pub addr: ::std::net::SocketAddr,
            pub rods_zone: String,
            pub instance: Option<String>,
        }


        impl packe::bosd::Serialiazable for #input_struct_name { }
        impl packe::bosd::xml::XMLSerializable for #input_struct_name {
            fn to_xml(&self, sink: &mut Vec<u8>)
                -> std::result::Result<usize, rods_prot_msg::error::errors::IrodsError>
            {
                use packe::{tag, tag_fmt};
                use ::std::io::Write;

                let mut cursor = ::std::io::Cursor::new(sink);
                let mut writer = ::quick_xml::Writer::new(&mut cursor);

                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("ExecMyRuleInp_PI")))?;

                writer.write_event(::quick_xml::events::Event::Start(
                    ::quick_xml::events::BytesStart::new("myRule")))?;
                writer.write_event(::quick_xml::events::Event::Text(
                    ::quick_xml::events::BytesText::new(#rule_body)))?;
                writer.write_event(::quick_xml::events::Event::End(
                    ::quick_xml::events::BytesEnd::new("myRule")))?;


                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("RHostAddr_PI")))?;
                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("hostAddr")))?;
                ::std::write!(writer.get_mut(), "{}", self.addr.ip())?;
                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("hostAddr")))?;

                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("rodsZone")))?;
                writer.write_event(::quick_xml::events::Event::Text(
                    ::quick_xml::events::BytesText::new(self.rods_zone.as_str())))?;
                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("rodsZone")))?;

                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("port")))?;
                ::std::write!(writer.get_mut(), "{}", self.addr.port())?;
                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("port")))?;

                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("dummyInt")))?;
                writer.write_event(::quick_xml::events::Event::Text(
                    ::quick_xml::events::BytesText::new("0")))?;
                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("dummyInt")))?;

                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("RHostAddr_PI")))?;

                if let Some(inst) = &self.instance {
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("KeyValPair_PI")))?;
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("ssLen")))?;
                    writer.write_event(::quick_xml::events::Event::Text(
                        ::quick_xml::events::BytesText::new("1")))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("ssLen")))?;
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("keyWord")))?;
                    writer.write_event(::quick_xml::events::Event::Text(
                        ::quick_xml::events::BytesText::new("instance_name")))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("keyWord")))?;
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("svalue")))?;
                    writer.write_event(::quick_xml::events::Event::Text(
                        ::quick_xml::events::BytesText::new(&self.instance)))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("svalue")))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("KeyValPair_PI")))?;
                } else {
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("KeyValPair_PI")))?;
                    writer.write_event(::quick_xml::events::Event::Start(
                            ::quick_xml::events::BytesStart::new("ssLen")))?;
                    writer.write_event(::quick_xml::events::Event::Text(
                        ::quick_xml::events::BytesText::new("0")))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("ssLen")))?;
                    writer.write_event(::quick_xml::events::Event::End(
                            ::quick_xml::events::BytesEnd::new("KeyValPair_PI")))?;
                }


                writer.write_event(::quick_xml::events::Event::Start(
                        ::quick_xml::events::BytesStart::new("MsParamArray_PI")))?;


                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("MsParamArray_PI")))?;


                writer.write_event(::quick_xml::events::Event::End(
                        ::quick_xml::events::BytesEnd::new("ExecMyRuleInp_PI")))?;


                Ok(cursor.position() as usize)
            }
        }
    };

    TokenStream::from(out)
}
