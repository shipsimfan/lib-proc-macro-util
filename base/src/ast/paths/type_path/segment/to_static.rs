use crate::ast::TypePathSegment;

impl<'a> TypePathSegment<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypePathSegment<'static> {
        TypePathSegment {
            ident: self.ident.into_static(),
            generics: self
                .generics
                .map(|(separator, generics)| (separator, generics.into_static())),
        }
    }
}
