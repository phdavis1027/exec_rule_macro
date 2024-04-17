use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, Field, Fields, ItemStruct, LitStr};

use crate::Rule;

pub(crate) fn write_tag<P, Q>(name: P, text: Q) -> proc_macro2::TokenStream
where
    P: quote::ToTokens,
    Q: quote::ToTokens,
{
    quote! {
        writer.write_event(::quick_xml::events::Event::Start(
                ::quick_xml::events::BytesStart::new(#name)))?;
        writer.write_event(::quick_xml::events::Event::Text(
                ::quick_xml::events::BytesText::new(#text)))?;
        writer.write_event(::quick_xml::events::Event::End(
                ::quick_xml::events::BytesEnd::new(#name)))?;
    }
}

pub(crate) fn write_tag_fmt<P, Q>(name: P, fmt: LitStr, text: Q) -> proc_macro2::TokenStream
where
    P: quote::ToTokens,
    Q: quote::ToTokens,
{
    quote! {
        writer.write_event(::quick_xml::events::Event::Start(
                ::quick_xml::events::BytesStart::new(#name)))?;
        ::std::write!(writer.get_mut(), #fmt, #text)?;
        writer.write_event(::quick_xml::events::Event::End(
                ::quick_xml::events::BytesEnd::new(#name)))?;
    }
}

pub(crate) fn write_start<P>(name: P) -> proc_macro2::TokenStream
where
    P: quote::ToTokens,
{
    quote! {
        writer.write_event(::quick_xml::events::Event::Start(
                ::quick_xml::events::BytesStart::new(#name)))?;
    }
}

pub(crate) fn write_end<P>(name: P) -> proc_macro2::TokenStream
where
    P: quote::ToTokens,
{
    quote! {
        writer.write_event(::quick_xml::events::Event::End(
                ::quick_xml::events::BytesEnd::new(#name)))?;
    }
}

pub(crate) fn expand_serialization_impl(rule: &Rule) -> proc_macro2::TokenStream {
    let write_commands = expand_write_commands(rule);
    let struct_name = &rule.receiver.ident;

    quote! {
        impl ::packe::bosd::Serialiazable for #struct_name { }
        impl ::packe::bosd::xml::XMLSerializable for #struct_name {
            fn to_xml(&self, sink: &mut Vec<u8>)
                -> std::result::Result<usize, rods_prot_msg::error::errors::IrodsError>
            {
                use packe::{tag, tag_fmt};
                use ::std::io::Write;

                let mut cursor = ::std::io::Cursor::new(sink);
                let mut writer = ::quick_xml::Writer::new(&mut cursor);

                #( #write_commands )*

                Ok(cursor.position() as usize)
            }
        }
    }
}

fn expand_write_commands(rule: &Rule) -> Vec<proc_macro2::TokenStream> {
    vec![
        write_start("ExecMyRuleInp_PI"),
        write_rule_body(&rule.receiver, &rule.body),
        write_rhost_addr(),
        write_kvp(),
        write_tag(
            "outParamDesc",
            rule.output
                .path
                .segments
                .iter()
                .last()
                .unwrap()
                .ident
                .to_string()
                .as_str(),
        ),
        write_param_array(rule),
        write_end("ExecMyRuleInp_PI"),
    ]
}

fn write_param_array(rule: &Rule) -> proc_macro2::TokenStream {
    let start = write_start("MsParamArray_PI");
    let end = write_end("MsParamArray_PI");

    let opr_type = write_tag("oprType", "0");

    quote! {
            #start

    //        #fields_len
            #opr_type

    //       #( #fields )*

            #end
        }
    .into()
}

fn write_rule_body(input: &ItemStruct, body: &LitStr) -> proc_macro2::TokenStream {
    let body = format!(
        "\
    @external
    {} {{
        {}
    }}
   ",
        input.ident.to_string(),
        body.value()
    );

    write_tag("myRule", body.as_str())
}

fn write_rhost_addr() -> proc_macro2::TokenStream {
    let start = write_start("RHostAddr_PI");

    let host_addr = write_tag_fmt(
        "hostAddr",
        LitStr::new("{}", Span::call_site()),
        quote! {
            self.addr.ip()
        },
    );

    let rods_zone = write_tag(
        "rodsZone",
        quote! {
            self.rods_zone.as_str()
        },
    );

    let port = write_tag_fmt(
        "port",
        LitStr::new("{}", Span::call_site()),
        quote! {
            self.addr.port()
        },
    );

    let dummy_int = write_tag("dummyInt", "0");

    let end = write_end("RHostAddr_PI");

    quote! {
        #start

        #host_addr
        #rods_zone
        #port
        #dummy_int

        #end
    }
}

fn write_kvp() -> proc_macro2::TokenStream {
    let start = write_start("KeyValPair_PI");

    let ss_len_zero = write_tag("ssLen", "0");
    let ss_len_one = write_tag("ssLen", "1");

    let key_word = write_tag("keyWord", "instance_name");

    let svalue = write_tag(
        "svalue",
        quote! {
            instance.as_str()
        },
    );

    let end = write_end("KeyValPair_PI");

    quote! {
        #start

        match &self.rule_engine_instance {
            Some(instance) => {
                #ss_len_one
                #key_word
                #svalue
            },
            None => {
                #ss_len_zero
            }
        }

        #end
    }
}
