use super::Punctuated;
use crate::tokens::{Comma, LeftTriangle, RightTriangle};

mod angle_brackets;
mod argument;
mod associated_constant;
mod associated_type;
mod bound;
mod constant_parameter;
mod constraint;
mod lifetime_parameter;
mod parameter;
mod parentheses;
mod type_parameter;
mod r#where;

pub use angle_brackets::AngleBracketGenerics;
pub use argument::GenericArgument;
pub use associated_constant::AssociatedConstant;
pub use associated_type::AssociatedType;
pub use bound::{BoundLifetimes, TraitBound, TraitBoundModifier, TypeParameterBound};
pub use constant_parameter::ConstantParameter;
pub use constraint::Constraint;
pub use lifetime_parameter::LifetimeParameter;
pub use parameter::GenericParameter;
pub use parentheses::ParenthesisGenerics;
pub use r#where::{PredicateLifetime, PredicateType, WhereClause, WherePredicate};
pub use type_parameter::TypeParameter;

#[derive(Clone)]
pub struct Generics {
    pub left_triangle: Option<LeftTriangle>,
    pub params: Punctuated<GenericParameter, Comma>,
    pub right_triangle: Option<RightTriangle>,
    pub where_clause: Option<WhereClause>,
}
