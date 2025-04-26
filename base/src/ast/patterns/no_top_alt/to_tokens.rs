use crate::{ast::PatternNoTopAlt, Generator, ToTokens};

impl<'a> ToTokens for PatternNoTopAlt<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            PatternNoTopAlt::Range(range) => range.to_tokens(generator),
        }
    }
}
