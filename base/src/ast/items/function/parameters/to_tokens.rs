use crate::{ast::items::FunctionParameters, Generator, ToTokens};

impl<'a> ToTokens for FunctionParameters<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            FunctionParameters::OnlySelf { _self, comma } => {
                _self.to_tokens(generator);
                comma.to_tokens(generator);
            }
            FunctionParameters::Normal {
                _self,
                first,
                remaining,
                ending,
            } => {
                _self.to_tokens(generator);
                first.to_tokens(generator);
                remaining.to_tokens(generator);
                ending.to_tokens(generator);
            }
        }
    }
}
