use crate::ast::items::CrateRef;
use std::borrow::Cow;

impl<'a> CrateRef<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> CrateRef<'static> {
        match self {
            CrateRef::Identifier(identifier) => CrateRef::Identifier(match identifier {
                Cow::Owned(owned) => Cow::Owned(owned),
                Cow::Borrowed(borrowed) => Cow::Owned(borrowed.clone()),
            }),
            CrateRef::_Self(_self) => CrateRef::_Self(_self),
        }
    }
}
