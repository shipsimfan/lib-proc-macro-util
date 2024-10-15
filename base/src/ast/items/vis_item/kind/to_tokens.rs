use crate::{ast::VisItemKind, Generator, ToTokens};

impl<'a> ToTokens for VisItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            VisItemKind::Module(module) => module.to_tokens(generator),
            VisItemKind::ExternCrate(extern_crate) => extern_crate.to_tokens(generator),
        }
    }
}
