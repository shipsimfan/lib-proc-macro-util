use crate::ast::types::TraitObjectType;

impl<'a> TraitObjectType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TraitObjectType<'static> {
        TraitObjectType {
            r#dyn: self.r#dyn,
            bounds: self.bounds.into_static(),
        }
    }
}
