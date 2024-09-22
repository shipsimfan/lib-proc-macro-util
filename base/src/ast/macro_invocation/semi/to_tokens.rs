use crate::{ast::MacroInvocationSemi, Generator, ToTokens};

impl<'a> ToTokens for MacroInvocationSemi<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            MacroInvocationSemi::ParenthesesOrBracket(path, exclamation, group, semi) => {
                path.to_tokens(generator);
                exclamation.to_tokens(generator);
                group.to_tokens(generator);
                semi.to_tokens(generator);
            }
            MacroInvocationSemi::Brace(path, exclamation, group) => {
                path.to_tokens(generator);
                exclamation.to_tokens(generator);
                group.to_tokens(generator);
            }
        }
    }
}
