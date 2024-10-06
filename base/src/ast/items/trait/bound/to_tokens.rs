use crate::{ast::TraitBound, Generator, ToTokens};

fn to_tokens(bound: TraitBound, generator: &mut Generator) {
    bound.question.to_tokens(generator);
    bound.for_lifetimes.to_tokens(generator);
    bound.path.to_tokens(generator);
}

impl<'a> ToTokens for TraitBound<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        if self.delimited {
            to_tokens(self, &mut generator.group_parenthesis());
        } else {
            to_tokens(self, generator);
        }
    }
}
