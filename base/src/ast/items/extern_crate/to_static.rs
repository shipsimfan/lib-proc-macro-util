use crate::ast::items::ExternCrate;

impl<'a> ExternCrate<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExternCrate<'static> {
        ExternCrate {
            r#extern: self.r#extern,
            krate: self.krate,
            crate_ref: self.crate_ref.into_static(),
            as_clause: self
                .as_clause
                .map(|(r#as, identifier)| (r#as, identifier.into_static())),
        }
    }
}
