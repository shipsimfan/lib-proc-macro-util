use crate::ast::InnerAttribute;

impl<'a> InnerAttribute<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> InnerAttribute<'static> {
        InnerAttribute {
            hash: self.hash,
            exclamation: self.exclamation,
            attr: self.attr.into_static(),
        }
    }
}
