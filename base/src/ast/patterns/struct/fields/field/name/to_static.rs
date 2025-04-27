use crate::ast::patterns::StructPatternFieldName;
use std::borrow::Cow;

impl<'a> StructPatternFieldName<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPatternFieldName<'static> {
        match self {
            StructPatternFieldName::Index(index, colon, pattern) => StructPatternFieldName::Index(
                Cow::Owned(match index {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                }),
                colon,
                Box::new(pattern.into_static()),
            ),
            StructPatternFieldName::IdentifierPattern(identifier, colon, pattern) => {
                StructPatternFieldName::IdentifierPattern(
                    Cow::Owned(match identifier {
                        Cow::Owned(owned) => owned,
                        Cow::Borrowed(borrowed) => borrowed.clone(),
                    }),
                    colon,
                    Box::new(pattern.into_static()),
                )
            }
            StructPatternFieldName::Identifier(r#ref, r#mut, identifier) => {
                StructPatternFieldName::Identifier(
                    r#ref,
                    r#mut,
                    Cow::Owned(match identifier {
                        Cow::Owned(owned) => owned,
                        Cow::Borrowed(borrowed) => borrowed.clone(),
                    }),
                )
            }
        }
    }
}
