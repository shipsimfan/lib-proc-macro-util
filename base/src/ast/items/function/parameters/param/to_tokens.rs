use crate::{ast::items::FunctionParam, Generator, ToTokens};

impl<'a> ToTokens for FunctionParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            FunctionParam::Variadic { attributes, dots } => {
                attributes.to_tokens(generator);
                dots.to_tokens(generator);
            }
            FunctionParam::Pattern {
                attributes,
                pattern,
                colon,
                r#type,
            } => {
                attributes.to_tokens(generator);
                pattern.to_tokens(generator);
                colon.to_tokens(generator);
                r#type.to_tokens(generator);
            }
        }
    }
}
