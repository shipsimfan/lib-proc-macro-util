use crate::{ast::VisItemKind, Generator, ToTokens};

impl<'a> ToTokens for VisItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            VisItemKind::Module(module) => module.to_tokens(generator),
        }
    }
}
