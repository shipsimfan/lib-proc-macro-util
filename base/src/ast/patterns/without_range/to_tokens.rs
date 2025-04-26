use crate::{ast::PatternWithoutRange, Generator, ToTokens};

impl<'a> ToTokens for PatternWithoutRange<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            PatternWithoutRange::Literal(literal) => literal.to_tokens(generator),
        }
    }
}
