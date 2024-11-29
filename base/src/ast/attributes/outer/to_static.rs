use crate::ast::OuterAttribute;

impl<'a> OuterAttribute<'a> {
    pub fn into_static(self) -> OuterAttribute<'static> {
        OuterAttribute {
            hash: self.hash,
            attr: self.attr.into_static(),
        }
    }
}
