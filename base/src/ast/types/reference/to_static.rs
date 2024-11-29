use crate::ast::{types::ReferenceType, Lifetime};

impl<'a> ReferenceType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ReferenceType<'static> {
        ReferenceType {
            reference: self.reference,
            lifetime: self.lifetime.map(Lifetime::into_static),
            r#mut: self.r#mut,
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
