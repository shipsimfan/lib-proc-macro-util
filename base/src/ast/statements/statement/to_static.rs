use crate::ast::Statement;

impl<'a> Statement<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Statement<'static> {
        match self {
            Statement::Item(item) => Statement::Item(item.into_static()),
            Statement::Let(r#let) => Statement::Let(r#let.into_static()),
            Statement::Expression(expression) => Statement::Expression(expression.into_static()),
            Statement::MacroInvocation(macro_invocation) => {
                Statement::MacroInvocation(macro_invocation.into_static())
            }
        }
    }
}
