use crate::ast::items::EnumItemDiscriminant;

impl<'a> EnumItemDiscriminant<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> EnumItemDiscriminant<'static> {
        EnumItemDiscriminant {
            equals: self.equals,
            expression: self.expression.into_static(),
        }
    }
}
