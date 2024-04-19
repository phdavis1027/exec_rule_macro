use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, Field, Fields, FieldsNamed, Ident, ItemStruct, LitStr};

use crate::{arg::Args, serialize::expand_serialization_impl};

pub(crate) struct Rule {
    // These fields will always be named, which
    // we guarantee in `try_new`
    pub(crate) receiver: ItemStruct,

    // The output struct that will be deserialized
    // from the server's response
    pub(crate) output: Ident,

    // The rule language that will be executed
    pub(crate) body: LitStr,
}

impl ToTokens for Rule {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        match expand_serialization_impl(self) {
            Ok(ts) => {
                out.extend(ts);
            }
            Err(e) => {
                out.extend(e.to_compile_error());
                return;
            }
        }
        let extended_receiver = self.add_extras_to_receiver();
        out.extend(extended_receiver.to_token_stream());

        out.extend(self.rule_trait_impl());
    }
}

impl Rule {
    fn rule_trait_impl(&self) -> proc_macro2::TokenStream {
        let receiver = &self.receiver.ident;
        let output = &self.output;
        let output = quote! {
            ::irods_client::exec_rule::#output
        };

        quote! {
            #[automatically_derived]
            impl ::irods_client::exec_rule::Rule for #receiver {
                type Output = #output;

                async fn execute<'c, T, C>(
                    &self,
                    conn: &'c mut ::irods_client::connection::Connection<T, C>,
                ) -> ::std::result::Result<
                    Self::Output, ::irods_client::error::errors::IrodsError
                >
                where
                    T: ::irods_client::bosd::ProtocolEncoding,
                    C: ::irods_client::reexports::tokio::io::AsyncRead + ::irods_client::reexports::tokio::io::AsyncWrite,
                    C: ::std::marker::Unpin + ::std::marker::Send
                {

                    conn.send_header_then_msg(
                        self,
                        ::irods_client::msg::header::MsgType::RodsApiReq,
                        ::irods_client::common::APN::ExecMyRule as i32
                    ).await?;

                    let (_, out) = conn.get_header_and_msg::<#output>().await?;
                    Ok(out)
                }
            }
        }
    }

    fn add_extras_to_receiver(&self) -> ItemStruct {
        let mut receiver = self.receiver.clone();
        let FieldsNamed {
            brace_token,
            mut named,
        } = match receiver.fields {
            Fields::Named(fields) => fields,
            _ => unreachable!(), // We already checked that we had named
                                 // fields in `try_new`
        };

        let addr_field: Field = syn::parse_quote! {
            #[builder(setter(name = "addr"))]
            pub __iRODS__EXEC__RULE__addr__: ::std::net::SocketAddr
        };

        let instance_field: Field = syn::parse_quote! {
            #[builder(setter(name = "instance"))]
            pub __iRODS__EXEC__RULE__rule_engine_instance__: ::std::option::Option<::std::string::String>
        };

        let zone_field: Field = syn::parse_quote! {
            #[builder(setter(name = "rods_zone"))]
            pub __iRODS__EXEC__RULE__rods_zone__: ::std::option::Option<::std::string::String>
        };

        named.extend(vec![addr_field, instance_field, zone_field]);

        receiver.fields = Fields::Named(FieldsNamed { brace_token, named });

        let builder_derive: Attribute = syn::parse_quote! {
            #[derive(::irods_client::reexports::derive_builder::Builder)] // Use reexport
        };

        receiver.attrs.push(builder_derive);

        receiver
    }

    pub(crate) fn try_new(args: Args, receiver: ItemStruct) -> syn::Result<Self> {
        let Fields::Named(_) = receiver.fields else {
            return Err(syn::Error::new(
                receiver.span(),
                "expected a struct with named fields",
            ));
        };

        let output = Ident::new(args.output.value().as_str(), args.output.span());

        Ok(Self {
            receiver,
            output,
            body: args.body,
        })
    }
}
