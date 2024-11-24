use crate::{ast::PatternNoTopAlt, Generator, ToTokens};

impl ToTokens for PatternNoTopAlt {
    fn to_tokens(self, generator: &mut Generator) {
        todo!("PatternNoTopAlt::to_tokens")
    }
}
