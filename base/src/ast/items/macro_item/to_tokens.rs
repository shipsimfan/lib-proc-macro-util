use crate::{ast::MacroItem, Generator, ToTokens};

impl ToTokens for MacroItem {
    fn to_tokens(self, generator: &mut Generator) {
        todo!()
    }
}
