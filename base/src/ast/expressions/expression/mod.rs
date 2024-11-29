use crate::ast::OuterAttribute;

mod from;
mod kind;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use kind::ExpressionKind;

/// A construct that evaluates to a value, and it can be used to perform computations, manipulate
/// data, or control the flow of a program
#[derive(Debug, Clone)]
pub struct Expression<'a> {
    /// Attributes affecting this expression
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The kind of expression
    pub kind: ExpressionKind<'a>,
}
