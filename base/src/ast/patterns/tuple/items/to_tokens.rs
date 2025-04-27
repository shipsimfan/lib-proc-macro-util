use crate::{ast::patterns::TuplePatternItems, Generator, ToTokens};

impl<'a> ToTokens for TuplePatternItems<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TuplePatternItems::Single(pattern, comma) => {
                pattern.to_tokens(generator);
                comma.to_tokens(generator);
            }
            TuplePatternItems::Rest(rest) => rest.to_tokens(generator),
            TuplePatternItems::Multiple {
                first,
                first_comma,
                second,
                remaining,
                last,
            } => {
                first.to_tokens(generator);
                first_comma.to_tokens(generator);
                second.to_tokens(generator);
                remaining.to_tokens(generator);
                last.to_tokens(generator);
            }
        }
    }
}
