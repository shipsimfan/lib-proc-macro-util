use crate::ast::types::ImplTraitTypeOneBound;

impl<'a> ImplTraitTypeOneBound<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ImplTraitTypeOneBound<'static> {
        ImplTraitTypeOneBound {
            r#impl: self.r#impl,
            bound: self.bound.into_static(),
        }
    }
}
