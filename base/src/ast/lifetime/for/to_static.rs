use crate::ast::ForLifetimes;

impl<'a> ForLifetimes<'a> {
    pub fn into_static(self) -> ForLifetimes<'static> {
        ForLifetimes {
            r#for: self.r#for,
            generics: self.generics.into_static(),
        }
    }
}
