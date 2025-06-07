use crate::ast::{PathIdentSegment, SimplePathSegment, TypePathSegment};

impl<'a> SimplePathSegment<'a> {
    /// Convert this segment into a [`TypePathSegment`]
    pub fn to_type_path_segment(&self) -> TypePathSegment<'a> {
        TypePathSegment {
            ident: match self {
                SimplePathSegment::Identifier(identifier) => {
                    PathIdentSegment::Identifier(identifier.clone())
                }
                SimplePathSegment::Crate(krate) => PathIdentSegment::_Crate(*krate),
                SimplePathSegment::_Self(_self) => PathIdentSegment::_LowerSelf(*_self),
                SimplePathSegment::Super(_super) => PathIdentSegment::Super(*_super),
                SimplePathSegment::DollarCrate(dollar, krate) => {
                    PathIdentSegment::DollarCrate(*dollar, *krate)
                }
            },
            generics: None,
        }
    }
}
