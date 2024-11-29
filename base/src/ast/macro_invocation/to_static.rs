use crate::ast::MacroInvocation;
use std::borrow::Cow;

impl<'a> MacroInvocation<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MacroInvocation<'static> {
        MacroInvocation {
            path: self.path.into_static(),
            exclamation: self.exclamation,
            group: Cow::Owned(match self.group {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
        }
    }
}
