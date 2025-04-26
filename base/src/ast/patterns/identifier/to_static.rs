use crate::ast::patterns::IdentifierPattern;
use std::borrow::Cow;

impl<'a> IdentifierPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> IdentifierPattern<'static> {
        IdentifierPattern {
            r#ref: self.r#ref,
            r#mut: self.r#mut,
            identifier: Cow::Owned(match self.identifier {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
            pattern: self
                .pattern
                .map(|(r#as, pattern)| (r#as, Box::new(pattern.into_static()))),
        }
    }
}
