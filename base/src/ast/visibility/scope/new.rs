use crate::{
    ast::{SimplePath, VisibilityScope},
    Token,
};

impl<'a> VisibilityScope<'a> {
    /// Creates a new [`VisibilityScope::Crate`]
    pub fn new_crate() -> Self {
        VisibilityScope::Crate(Token![crate]())
    }

    /// Creates a new [`VisibilityScope::_Self`]
    pub fn new_self() -> Self {
        VisibilityScope::_Self(Token![self]())
    }

    /// Creates a new [`VisibilityScope::Super`]
    pub fn new_super() -> Self {
        VisibilityScope::Super(Token![super]())
    }

    /// Creates a new [`VisibilityScope::Crate`]
    pub fn new_path<T: Into<SimplePath<'a>>>(path: T) -> Self {
        VisibilityScope::Path(Token![in](), path.into())
    }
}
