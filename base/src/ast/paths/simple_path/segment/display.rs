use crate::ast::SimplePathSegment;

impl<'a> std::fmt::Display for SimplePathSegment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimplePathSegment::Identifier(identifier) => identifier.fmt(f),
            SimplePathSegment::OwnedIdentifier(identifier) => identifier.fmt(f),
            SimplePathSegment::Crate(krate) => krate.fmt(f),
            SimplePathSegment::_Self(_self) => _self.fmt(f),
            SimplePathSegment::Super(_super) => _super.fmt(f),
            SimplePathSegment::DollarCrate(dollar, krate) => {
                dollar.fmt(f)?;
                krate.fmt(f)
            }
        }
    }
}
