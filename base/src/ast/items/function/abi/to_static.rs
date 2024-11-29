use crate::ast::Abi;
use std::borrow::Cow;

impl<'a> Abi<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Abi<'static> {
        Abi(Cow::Owned(match self.0 {
            Cow::Borrowed(borrowed) => borrowed.clone(),
            Cow::Owned(owned) => owned,
        }))
    }
}
