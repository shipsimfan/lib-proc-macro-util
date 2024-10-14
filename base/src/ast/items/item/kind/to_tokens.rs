use crate::{ast::ItemKind, Generator, ToTokens};

impl<'a> ToTokens for ItemKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ItemKind::Vis(vis_item) => vis_item.to_tokens(generator),
            ItemKind::Macro(macro_item) => macro_item.to_tokens(generator),
        }
    }
}
