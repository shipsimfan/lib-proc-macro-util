use crate::ast::{expressions::BlockExpression, InnerAttribute};

impl<'a> BlockExpression<'a> {
    pub fn into_static(self) -> BlockExpression<'static> {
        BlockExpression {
            attributes: self
                .attributes
                .into_iter()
                .map(InnerAttribute::into_static)
                .collect(),
            statements: self.statements,
            end: self.end.map(|end| Box::new(end.into_static())),
        }
    }
}
