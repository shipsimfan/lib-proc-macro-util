use crate::ast::types::ImplTraitType;

impl<'a> ImplTraitType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ImplTraitType<'static> {
        ImplTraitType {
            r#impl: self.r#impl,
            bounds: self.bounds.into_static(),
        }
    }
}
