use crate::ast::expressions::{ClosureExpression, ClosureParameters};

impl<'a> ClosureExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ClosureExpression<'static> {
        ClosureExpression {
            r#async: self.r#async,
            r#move: self.r#move,
            leading: self.leading,
            parameters: self.parameters.map(ClosureParameters::into_static),
            trailing: self.trailing,
            body: self.body.into_static(),
        }
    }
}
