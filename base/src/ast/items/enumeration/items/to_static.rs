use crate::ast::items::EnumItems;

impl<'a> EnumItems<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> EnumItems<'static> {
        EnumItems {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, item)| (comma, item.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
