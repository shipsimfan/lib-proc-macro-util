use crate::{ast::expressions::ArrayElements, Generator, ToTokens};

impl<'a> ToTokens for ArrayElements<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ArrayElements::Every {
                first,
                remaining,
                last,
            } => {
                first.to_tokens(generator);
                remaining.to_tokens(generator);
                last.to_tokens(generator);
            }
            ArrayElements::Uniform {
                element,
                semi,
                count,
            } => {
                element.to_tokens(generator);
                semi.to_tokens(generator);
                count.to_tokens(generator);
            }
        }
    }
}
