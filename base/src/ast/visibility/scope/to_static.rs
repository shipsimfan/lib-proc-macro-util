use crate::ast::VisibilityScope;

impl<'a> VisibilityScope<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> VisibilityScope<'static> {
        match self {
            VisibilityScope::Crate(krate) => VisibilityScope::Crate(krate),
            VisibilityScope::_Self(_self) => VisibilityScope::_Self(_self),
            VisibilityScope::Super(_super) => VisibilityScope::Super(_super),
            VisibilityScope::Path(r#in, path) => VisibilityScope::Path(r#in, path.into_static()),
        }
    }
}
