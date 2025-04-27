use crate::ast::PathInExpression;

mod items;

mod parse;
mod to_static;
mod to_tokens;

pub use items::TupleStructItems;

/// Tuple struct patterns match tuple struct and enum values that match all criteria defined by its
/// subpatterns.
#[derive(Debug, Clone)]
pub struct TupleStructPattern<'a> {
    /// The path to the tuple struct
    pub path: PathInExpression<'a>,

    /// The items the be matched in the tuple struct
    pub items: Option<TupleStructItems<'a>>,
}
