use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::rule_input::RuleInput;

pub(crate) fn write_tag<P, Q>(name: P, text: Q) -> TokenStream
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

pub(crate) fn write_start<P>(name: P) -> TokenStream
where
    P: quote::ToTokens,
{
    quote! {
        writer.write_event(::quick_xml::events::Event::Start(
                ::quick_xml::events::BytesStart::new(#name)))?;
    }
}

pub(crate) fn expand_serialize(input: &RuleInput, input_struct_name: Ident) -> TokenStream {
    let write_commands = expand_write_commands(input);

    quote! {
        impl ::packe::bosd::Serialiazable for #input_struct_name { }
        impl ::packe::bosd::xml::XMLSerializable for #input_struct_name {
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

fn expand_write_commands(input: &RuleInput) -> Vec<TokenStream> {
    unimplemented!()
}
