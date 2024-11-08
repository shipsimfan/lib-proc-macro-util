use crate::{ast::VisItemKind, Generator, ToTokens};

impl<'a> ToTokens for VisItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            VisItemKind::Module(module) => module.to_tokens(generator),
            VisItemKind::ExternCrate(extern_crate) => extern_crate.to_tokens(generator),
            VisItemKind::Use(r#use) => r#use.to_tokens(generator),
            VisItemKind::Function(function) => function.to_tokens(generator),
            VisItemKind::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
