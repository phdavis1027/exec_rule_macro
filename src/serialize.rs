use proc_macro2::Span;
use quote::quote;
use syn::{Ident, LitStr};

use crate::rule::Rule;
use crate::write_utils::*;

pub(crate) fn expand_serialization_impl(rule: &Rule) -> syn::Result<proc_macro2::TokenStream> {
    let write_commands = expand_write_commands(rule)?;
    let struct_name = &rule.receiver.ident;

    Ok(quote! {
        #[automatically_derived]
        impl ::irods_client::bosd::Serialiazable for #struct_name { }

        #[automatically_derived]
        impl ::irods_client::bosd::xml::XMLSerializable for #struct_name {
            fn to_xml(&self, sink: &mut Vec<u8>)
                -> ::std::result::Result<usize, ::irods_client::error::errors::IrodsError>
            {
                use ::irods_client::{tag, tag_fmt};
                use ::std::io::Write;

                let mut cursor = ::std::io::Cursor::new(sink);
                let mut writer = ::irods_client::reexports::quick_xml::Writer::new(&mut cursor);

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
        write_tag("outParamDesc", serialize_output_name(&rule.output)?),
        write_param_array(rule)?,
        write_end("ExecMyRuleInp_PI"),
    ])
}

fn serialize_output_name(output: &syn::Ident) -> syn::Result<&'static str> {
    match output.to_string().as_str() {
        "ExecRuleOut" => Ok("execRuleOut"),
        _ => Err(syn::Error::new_spanned(output, "Unsupported output type")),
    }
}

fn write_param_array(rule: &Rule) -> syn::Result<proc_macro2::TokenStream> {
    let start = write_start("MsParamArray_PI");
    let end = write_end("MsParamArray_PI");

    let mut params = Vec::new();
    for field in &rule.receiver.fields {
        params.push(field_to_irods_param(field)?);
    }

    let params_len = write_tag("paramLen", format!("{}", params.len()).as_str());

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
            self.__iRODS__EXEC__RULE__addr__.ip()
        },
    );

    let some = write_tag(
        "rodsZone",
        quote! {
            zone.as_str()
        },
    );
    let none = write_tag("rodsZone", "");

    let rods_zone = quote! {
        match self.__iRODS__EXEC__RULE__rods_zone__ {
            Some(ref zone) => {
                #some
            },
            None => {
                #none
            }
        }
    };

    let port = write_tag_fmt(
        "port",
        LitStr::new("{}", Span::call_site()),
        quote! {
            self.__iRODS__EXEC__RULE__addr__.port()
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
    let end = write_end("KeyValPair_PI");

    let ss_len_zero = write_tag("ssLen", "0");
    let ss_len_one = write_tag("ssLen", "1");

    let key_word = write_tag("keyWord", "instance_name");

    let svalue = write_tag(
        "svalue",
        quote! {
            instance.as_str()
        },
    );

    quote! {
        #start

        match &self.__iRODS__EXEC__RULE__rule_engine_instance__ {
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
