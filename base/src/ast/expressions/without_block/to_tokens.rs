use crate::{ast::ExpressionWithoutBlock, Generator, ToTokens};

impl<'a> ToTokens for ExpressionWithoutBlock<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionWithoutBlock::Literal(literal) => literal.to_tokens(generator),
        }
    }
}
