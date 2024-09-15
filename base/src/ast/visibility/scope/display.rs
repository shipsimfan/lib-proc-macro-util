use crate::ast::VisibilityScope;

impl<'a> std::fmt::Display for VisibilityScope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisibilityScope::Crate(krate) => krate.fmt(f),
            VisibilityScope::_Self(_self) => _self.fmt(f),
            VisibilityScope::Super(_super) => _super.fmt(f),
            VisibilityScope::Path(_, path) => write!(f, "in {}", path),
        }
    }
}
