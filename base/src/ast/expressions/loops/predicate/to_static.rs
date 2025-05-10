use crate::ast::expressions::PredicateLoopExpression;

impl<'a> PredicateLoopExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PredicateLoopExpression<'static> {
        PredicateLoopExpression {
            r#while: self.r#while,
            r#let: self
                .r#let
                .map(|(r#let, pattern, eq)| (r#let, pattern.into_static(), eq)),
            condition: Box::new(self.condition.into_static()),
            block: self.block.into_static(),
        }
    }
}
