use crate::ast::{items::VisItem, Visibility};

impl<'a> VisItem<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> VisItem<'static> {
        VisItem {
            visibility: self.visibility.map(Visibility::into_static),
            kind: self.kind.into_static(),
        }
    }
}
