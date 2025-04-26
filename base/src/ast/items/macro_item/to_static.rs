use crate::ast::MacroItem;

impl<'a> MacroItem<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MacroItem<'static> {
        match self {
            MacroItem::MacroInvocationSemi(macro_invoke) => {
                MacroItem::MacroInvocationSemi(macro_invoke.into_static())
            }
        }
    }
}
