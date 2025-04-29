use crate::ast::expressions::IfExpression;

impl<'a> IfExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> IfExpression<'static> {
        IfExpression {
            r#if: self.r#if,
            r#let: self
                .r#let
                .map(|(r#let, pattern, equals)| (r#let, pattern.into_static(), equals)),
            condition: Box::new(self.condition.into_static()),
            block: self.block.into_static(),
            r#else: self
                .r#else
                .map(|(r#else, r#if)| (r#else, r#if.into_static())),
        }
    }
}
