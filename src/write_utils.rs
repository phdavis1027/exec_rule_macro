use quote::quote;
use syn::{spanned::Spanned, Field, Ident, LitStr, Type};

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

pub(crate) fn write_string_value(label: &Ident) -> proc_macro2::TokenStream {
    let start = write_start("STR_PI");
    let end = write_end("STR_PI");

    let value = write_tag(
        "myStr",
        quote! {
            self.#label.as_str()
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

pub(crate) fn write_byte_value(label: &Ident) -> proc_macro2::TokenStream {
    let start = write_start("CHAR_PI");
    let end = write_end("CHAR_PI");

    let value = write_tag(
        "myChar",
        quote! {
            self.#label
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

pub(crate) fn write_int16_value(label: &Ident) -> proc_macro2::TokenStream {
    let start = write_start("INT16_PI");
    let end = write_end("INT16_PI");

    let value = write_tag(
        "myInt",
        quote! {
            self.#label
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

pub(crate) fn write_int32_value(label: &Ident) -> proc_macro2::TokenStream {
    let start = write_start("INT_PI");
    let end = write_end("INT_PI");

    let value = write_tag(
        "myInt",
        quote! {
            self.#label
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

pub(crate) fn write_double_value(label: &Ident) -> proc_macro2::TokenStream {
    let start = write_start("DOUBLE_PI");
    let end = write_end("DOUBLE_PI");

    let value = write_tag(
        "myDouble",
        quote! {
            self.#label
        },
    );

    quote! {
        #start

        #value

        #end
    }
}

pub(crate) fn field_to_irods_param(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let start = write_start("MsParam_PI");
    let end = write_end("MsParam_PI");

    let label = write_tag("label", field.ident.as_ref().unwrap().to_string().as_str());

    let ty = match &field.ty {
        Type::Path(ref ty) => ty.clone(),
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "Only syn::Path types are supported for serialization.",
            ));
        }
    };

    let ty = ty.path.segments.last().unwrap().ident.to_string();

    let param_ty = match ty.as_str() {
        "String" => write_tag("type", LitStr::new("STR_PI", field.span())),
        "i32" => write_tag("type", LitStr::new("INT_PI", field.span())),
        "i16" => write_tag("type", LitStr::new("INT16_PI", field.span())),
        "f64" => write_tag("type", LitStr::new("DOUBLE_PI", field.span())),
        "u8" => write_tag("type", LitStr::new("CHAR_PI", field.span())),
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "Only i32, i16, f64, u8, and String types are currently mapped to Rule Language types",
            ));
        }
    };

    let value = match ty.as_str() {
        "String" => write_string_value(field.ident.clone().as_ref().unwrap()),
        "i32" => write_byte_value(&field.ident.clone().unwrap()),
        "i16" => write_int16_value(&field.ident.clone().unwrap()),
        "f64" => write_double_value(&field.ident.clone().unwrap()),
        "u8" => write_byte_value(&field.ident.clone().unwrap()),
        _ => {
            return Err(syn::Error::new(
                field.span(),
                "Only i32, i16, f64, u8, and String types are currently mapped to Rule Language types",
            ));
        }
    };

    Ok(quote! {
        #start

        #label
        #param_ty
        #value

        #end
    })
}
