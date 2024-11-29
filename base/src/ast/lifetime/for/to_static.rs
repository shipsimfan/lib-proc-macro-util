use crate::ast::ForLifetimes;

impl<'a> ForLifetimes<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ForLifetimes<'static> {
        ForLifetimes {
            r#for: self.r#for,
            generics: self.generics.into_static(),
        }
    }
}
