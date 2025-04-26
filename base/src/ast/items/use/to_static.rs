use crate::ast::items::UseDeclaration;

impl<'a> UseDeclaration<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> UseDeclaration<'static> {
        UseDeclaration {
            r#use: self.r#use,
            tree: self.tree.into_static(),
            semi: self.semi,
        }
    }
}
