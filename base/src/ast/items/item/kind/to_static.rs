use crate::ast::ItemKind;

impl<'a> ItemKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ItemKind<'static> {
        match self {
            ItemKind::Vis(vis_item) => ItemKind::Vis(vis_item.into_static()),
            ItemKind::Macro(r#macro) => ItemKind::Macro(r#macro.into_static()),
        }
    }
}
