use std::borrow::Cow;

use crate::ast::SimplePathSegment;

impl<'a> SimplePathSegment<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> SimplePathSegment<'static> {
        match self {
            SimplePathSegment::Identifier(identifier) => {
                SimplePathSegment::Identifier(Cow::Owned(match identifier {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
            SimplePathSegment::Crate(krate) => SimplePathSegment::Crate(krate),
            SimplePathSegment::_Self(_self) => SimplePathSegment::_Self(_self),
            SimplePathSegment::Super(_super) => SimplePathSegment::Super(_super),
            SimplePathSegment::DollarCrate(dollar, krate) => {
                SimplePathSegment::DollarCrate(dollar, krate)
            }
        }
    }
}
