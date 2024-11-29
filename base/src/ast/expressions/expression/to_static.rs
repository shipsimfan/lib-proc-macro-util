use crate::ast::{Expression, OuterAttribute};

impl<'a> Expression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Expression<'static> {
        Expression {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            kind: self.kind.into_static(),
        }
    }
}
