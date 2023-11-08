use crate::{
    ast::{AssociatedConstant, AssociatedType, Constraint, Expression, Lifetime, Type},
    parsing::Parse,
};

#[derive(Clone)]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Constant(Expression),
    AssociatedType(AssociatedType),
    AssociatedConstant(AssociatedConstant),
    Constraint(Constraint),
}

impl<'a> Parse<'a> for GenericArgument {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        todo!()
    }
}
