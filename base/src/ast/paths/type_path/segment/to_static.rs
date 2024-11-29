use crate::ast::TypePathSegment;

impl<'a> TypePathSegment<'a> {
    pub fn into_static(self) -> TypePathSegment<'static> {
        TypePathSegment {
            ident: self.ident,
            generics: self
                .generics
                .map(|(separator, generics)| (separator, generics.into_static())),
        }
    }
}
