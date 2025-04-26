use crate::{ast::patterns::RangePatternBound, Generator, ToTokens};

impl<'a> ToTokens for RangePatternBound<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            RangePatternBound::Literal(neg, literal) => {
                neg.to_tokens(generator);
                literal.to_tokens(generator);
            }
            RangePatternBound::Path(path) => path.to_tokens(generator),
        }
    }
}
