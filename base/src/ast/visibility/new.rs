use crate::{
    ast::{SimplePath, Visibility, VisibilityScope},
    Token,
};

impl<'a> Visibility<'a> {
    /// Creates a new [`Visibility`]
    pub const fn new_raw(r#pub: Token![pub], scope: Option<VisibilityScope<'a>>) -> Self {
        Visibility { r#pub, scope }
    }

    /// Creates a new [`Visibility`] with `scope`
    pub fn new<T: Into<VisibilityScope<'a>>>(scope: T) -> Self {
        Visibility::new_raw(Token![pub](), Some(scope.into()))
    }

    /// Creates a new [`Visibility`] with a public scope
    pub fn new_pub() -> Self {
        Visibility::new_raw(Token![pub](), None)
    }

    /// Creates a new [`Visibility`] with [`VisibilityScope::Crate`]
    pub fn new_crate() -> Self {
        Visibility::new(Token![crate]())
    }

    /// Creates a new [`Visibility`] with [`VisibilityScope::_Self`]
    pub fn new_self() -> Self {
        Visibility::new(Token![self]())
    }

    /// Creates a new [`Visibility`] with [`VisibilityScope::Super`]
    pub fn new_super() -> Self {
        Visibility::new(Token![super]())
    }

    /// Creates a new [`Visibility`] with [`VisibilityScope::Path`]
    pub fn new_path<T: Into<SimplePath<'a>>>(path: T) -> Self {
        Visibility::new(VisibilityScope::new_path(path))
    }
}
