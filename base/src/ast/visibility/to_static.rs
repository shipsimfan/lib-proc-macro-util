use crate::ast::{Visibility, VisibilityScope};

impl<'a> Visibility<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Visibility<'static> {
        Visibility {
            r#pub: self.r#pub,
            scope: self.scope.map(VisibilityScope::into_static),
        }
    }
}
