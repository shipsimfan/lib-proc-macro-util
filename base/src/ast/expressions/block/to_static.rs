use crate::ast::{expressions::BlockExpression, InnerAttribute, Statement};

impl<'a> BlockExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> BlockExpression<'static> {
        BlockExpression {
            attributes: self
                .attributes
                .into_iter()
                .map(InnerAttribute::into_static)
                .collect(),
            statements: self
                .statements
                .into_iter()
                .map(Statement::into_static)
                .collect(),
            end: self.end.map(|end| Box::new(end.into_static())),
        }
    }
}
