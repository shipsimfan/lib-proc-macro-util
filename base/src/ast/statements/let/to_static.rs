use crate::ast::{statements::LetStatement, OuterAttribute};

impl<'a> LetStatement<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LetStatement<'static> {
        LetStatement {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            r#let: self.r#let,
            pattern: self.pattern.into_static(),
            r#type: self
                .r#type
                .map(|(colon, r#type)| (colon, r#type.into_static())),
            expression: self.expression.map(|(equals, expression, r#else)| {
                (
                    equals,
                    expression.into_static(),
                    r#else.map(|(r#else, block)| (r#else, block.into_static())),
                )
            }),
        }
    }
}
