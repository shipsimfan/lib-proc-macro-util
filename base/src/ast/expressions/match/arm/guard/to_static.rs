use crate::ast::expressions::MatchArmGuard;

impl<'a> MatchArmGuard<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MatchArmGuard<'static> {
        MatchArmGuard {
            r#if: self.r#if,
            expression: self.expression.into_static(),
        }
    }
}
