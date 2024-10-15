use crate::{ast::MacroItem, Generator, ToTokens};

impl<'a> ToTokens for MacroItem<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            MacroItem::MacroInvocationSemi(macro_invocation) => {
                macro_invocation.to_tokens(generator)
            }
        }
    }
}
