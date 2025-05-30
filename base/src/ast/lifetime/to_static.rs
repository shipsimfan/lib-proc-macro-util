use crate::ast::Lifetime;
use std::borrow::Cow;

impl<'a> Lifetime<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Lifetime<'static> {
        match self {
            Lifetime::Identifier(quote, identifier) => Lifetime::Identifier(
                quote,
                Cow::Owned(match identifier {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }),
            ),
            Lifetime::Underscore(quote, underscore) => Lifetime::Underscore(quote, underscore),
        }
    }
}
