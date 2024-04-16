use syn::{parse::Parse, Token};

use crate::kw;

pub enum OutputType {
    ExecRuleOut,
}

impl OutputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputType::ExecRuleOut => "exec_rule_out",
        }
    }
}

impl Parse for OutputType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::exec_rule_out) {
            input.parse::<kw::exec_rule_out>()?;
            Ok(OutputType::ExecRuleOut)
        } else {
            Err(lookahead.error())
        }
    }
}
