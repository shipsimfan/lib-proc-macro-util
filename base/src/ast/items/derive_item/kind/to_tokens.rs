use crate::{ast::items::DeriveItemKind, Generator, ToTokens};

impl<'a> ToTokens for DeriveItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            DeriveItemKind::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
