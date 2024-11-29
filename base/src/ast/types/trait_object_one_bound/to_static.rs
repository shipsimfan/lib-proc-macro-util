use crate::ast::types::TraitObjectTypeOneBound;

impl<'a> TraitObjectTypeOneBound<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TraitObjectTypeOneBound<'static> {
        TraitObjectTypeOneBound {
            r#dyn: self.r#dyn,
            bound: self.bound.into_static(),
        }
    }
}
