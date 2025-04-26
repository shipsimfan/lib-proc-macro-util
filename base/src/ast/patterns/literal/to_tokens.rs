use crate::{ast::patterns::LiteralPattern, Generator, ToTokens};

impl<'a> ToTokens for LiteralPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            LiteralPattern::Literal(neg, literal) => {
                neg.to_tokens(generator);
                literal.to_tokens(generator);
            }
            LiteralPattern::True(r#true) => r#true.to_tokens(generator),
            LiteralPattern::False(r#false) => r#false.to_tokens(generator),
        }
    }
}
