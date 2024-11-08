use crate::{ast::ConstParamValue, Generator, ToTokens};

impl<'a> ToTokens for ConstParamValue<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ConstParamValue::Block(block) => block.to_tokens(generator),
            ConstParamValue::Identifier(identifier) => identifier.to_tokens(generator),
            ConstParamValue::Literal(minus, literal) => {
                minus.to_tokens(generator);
                literal.to_tokens(generator);
            }
        }
    }
}
