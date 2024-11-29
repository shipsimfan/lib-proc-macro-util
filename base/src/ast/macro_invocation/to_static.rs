use crate::ast::MacroInvocation;
use std::borrow::Cow;

impl<'a> MacroInvocation<'a> {
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
