use crate::ast::PathIdentSegment;
use std::borrow::Cow;

impl<'a> PathIdentSegment<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PathIdentSegment<'static> {
        match self {
            PathIdentSegment::Identifier(identifier) => {
                PathIdentSegment::Identifier(Cow::Owned(match identifier {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(borrowed) => borrowed.to_owned(),
                }))
            }
            PathIdentSegment::Super(_super) => PathIdentSegment::Super(_super),
            PathIdentSegment::_LowerSelf(_self) => PathIdentSegment::_LowerSelf(_self),
            PathIdentSegment::_UpperSelf(_self) => PathIdentSegment::_UpperSelf(_self),
            PathIdentSegment::_Crate(_crate) => PathIdentSegment::_Crate(_crate),
            PathIdentSegment::DollarCrate(dollar, _crate) => {
                PathIdentSegment::DollarCrate(dollar, _crate)
            }
        }
    }
}
