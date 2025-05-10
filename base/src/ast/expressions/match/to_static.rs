use crate::ast::{expressions::MatchExpression, InnerAttribute};

impl<'a> MatchExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MatchExpression<'static> {
        MatchExpression {
            r#match: self.r#match,
            scrutinee: Box::new(self.scrutinee.into_static()),
            attributes: self
                .attributes
                .into_iter()
                .map(InnerAttribute::into_static)
                .collect(),
            arms: self
                .arms
                .into_iter()
                .map(|(arm, arrow, expression, comma)| {
                    (arm.into_static(), arrow, expression.into_static(), comma)
                })
                .collect(),
        }
    }
}
