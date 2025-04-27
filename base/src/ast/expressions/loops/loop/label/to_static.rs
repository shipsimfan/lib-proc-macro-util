use crate::ast::expressions::LoopLabel;
use std::borrow::Cow;

impl<'a> LoopLabel<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LoopLabel<'static> {
        LoopLabel {
            quote: self.quote,
            name: Cow::Owned(match self.name {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
            colon: self.colon,
        }
    }
}
