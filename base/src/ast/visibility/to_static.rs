use crate::ast::{Visibility, VisibilityScope};

impl<'a> Visibility<'a> {
    pub fn into_static(self) -> Visibility<'a> {
        Visibility {
            r#pub: self.r#pub,
            scope: self.scope.map(VisibilityScope::into_static),
        }
    }
}
