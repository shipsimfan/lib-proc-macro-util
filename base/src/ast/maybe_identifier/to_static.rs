use crate::ast::MaybeIdentifier;
use std::borrow::Cow;

impl<'a> MaybeIdentifier<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MaybeIdentifier<'static> {
        match self {
            MaybeIdentifier::Identifier(identifier) => {
                MaybeIdentifier::Identifier(Cow::Owned(match identifier {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
            MaybeIdentifier::Underscore(underscore) => MaybeIdentifier::Underscore(underscore),
        }
    }
}
