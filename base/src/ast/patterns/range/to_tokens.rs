use crate::{ast::patterns::RangePattern, Generator, ToTokens};

impl<'a> ToTokens for RangePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            RangePattern::Exclusive(exclusive) => exclusive.to_tokens(generator),
            RangePattern::Inclusive(inclusive) => inclusive.to_tokens(generator),
            RangePattern::From(from) => from.to_tokens(generator),
            RangePattern::ToInclusive(to_inclusive) => to_inclusive.to_tokens(generator),
            RangePattern::Obsolete(obsolete) => obsolete.to_tokens(generator),
        }
    }
}
