use crate::ast::{expressions::StructExprField, OuterAttribute};

impl<'a> StructExprField<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExprField<'static> {
        StructExprField {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            name: self.name.into_static(),
            expression: self
                .expression
                .map(|(colon, expression)| (colon, Box::new(expression.into_static()))),
        }
    }
}
