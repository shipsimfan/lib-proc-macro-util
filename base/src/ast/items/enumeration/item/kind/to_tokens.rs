use crate::{ast::items::EnumItemKind, Generator, ToTokens};

impl<'a> ToTokens for EnumItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            EnumItemKind::Tuple(tuple) => tuple.to_tokens(&mut generator.group_parenthesis()),
            EnumItemKind::Struct(r#struct) => r#struct.to_tokens(&mut generator.group_brace()),
        }
    }
}
