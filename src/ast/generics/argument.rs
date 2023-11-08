use crate::ast::{AssociatedConstant, AssociatedType, Constraint, Expression, Lifetime, Type};

#[derive(Clone)]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Constant(Expression),
    AssociatedType(AssociatedType),
    AssociatedConstant(AssociatedConstant),
    Constraint(Constraint),
}
