use quote::{quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Field, Fields, FieldsNamed, Ident,
    ItemStruct, LitStr, TypePath,
};

use crate::{arg::Args, serialize::expand_serialization_impl};

pub(crate) struct Rule {
    // These fields will always be named, which
    // we guarantee in `try_new`
    pub(crate) receiver: ItemStruct,

    // The output struct that will be deserialized
    // from the server's response
    pub(crate) output: proc_macro2::TokenStream,

    // The rule language that will be executed
    pub(crate) body: LitStr,
}

impl ToTokens for Rule {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        let extended_receiver = self.add_extra_fields_to_receiver();
        match expand_serialization_impl(self) {
            Ok(serialization_impl) => {
                out.extend(serialization_impl);
            }
            Err(err) => {
                out.extend(err.to_compile_error());
                return;
            }
        }
    }
}

impl Rule {
    fn add_extra_fields_to_receiver(&self) -> ItemStruct {
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
            pub __iRODS__EXEC__RULE__addr__: ::std::net::SocketAddr
        };

        let instance_field: Field = syn::parse_quote! {
            pub __iRODS_EXEC__RULE__instance__: Option<String>
        };

        named.extend(vec![addr_field, instance_field]);

        receiver.fields = Fields::Named(FieldsNamed { brace_token, named });

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
        let output = quote_spanned! { output.span() =>
            ::packe::exec_rule::#output
        };

        Ok(Self {
            receiver,
            output,
            body: args.body,
        })
    }
}
