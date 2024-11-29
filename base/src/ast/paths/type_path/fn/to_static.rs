use crate::ast::TypePathFn;

impl<'a> TypePathFn<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypePathFn<'static> {
        TypePathFn {
            inputs: self.inputs,
            r#return: self
                .r#return
                .map(|(arrow, r#type)| (arrow, Box::new(r#type.into_static()))),
        }
    }
}
