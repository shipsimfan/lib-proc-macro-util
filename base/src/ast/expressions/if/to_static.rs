use crate::ast::expressions::IfExpression;

impl<'a> IfExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> IfExpression<'static> {
        IfExpression {
            r#if: self.r#if,
            r#let: self.r#let,
            condition: (),
            block: (),
            r#else: (),
        }
    }
}
