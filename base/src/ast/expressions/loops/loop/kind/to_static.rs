use crate::ast::expressions::LoopExpressionKind;

impl<'a> LoopExpressionKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LoopExpressionKind<'static> {
        match self {
            LoopExpressionKind::Iterator(iterator) => {
                LoopExpressionKind::Iterator(iterator.into_static())
            }
        }
    }
}
