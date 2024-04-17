use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, Field, Fields, Ident, ItemStruct, Lit, LitStr, Type};

use crate::write_utils::*;

use crate::rule::Rule;

pub(crate) fn expand_serialization_impl(rule: &Rule) -> syn::Result<proc_macro2::TokenStream> {
    println!("Got here");
    let write_commands = expand_write_commands(rule)?;
    let struct_name = &rule.receiver.ident;

    Ok(quote! {
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
    })
}

fn expand_write_commands(rule: &Rule) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    Ok(vec![
        write_start("ExecMyRuleInp_PI"),
        write_rule_body(rule),
        write_rhost_addr(),
        write_kvp(),
        write_tag("outParamDesc", &rule.output),
        write_param_array(rule)?,
        write_end("ExecMyRuleInp_PI"),
    ])
}

fn write_param_array(rule: &Rule) -> syn::Result<proc_macro2::TokenStream> {
    let start = write_start("MsParamArray_PI");
    let end = write_end("MsParamArray_PI");

    let mut params = Vec::new();
    for field in &rule.receiver.fields {
        params.push(field_to_irods_param(field)?);
    }

    println!("Got here");

    let params_len = write_tag_fmt("paramLen", LitStr::new("{}", rule.span()), params.len());

    let opr_type = write_tag("oprType", "0");

    Ok(quote! {
        #start

        #params_len
        #opr_type

        #( #params )*

        #end
    }
    .into())
}

fn write_rule_body(rule: &Rule) -> proc_macro2::TokenStream {
    let receiver = rule.receiver.ident.to_string();
    let body = rule.body.value();

    let body = format!(
        "\
    @external
    {} {{
        {}
    }}
   ",
        receiver, body
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
