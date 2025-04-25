use crate::ast::OuterAttribute;

mod from;
mod kind;
mod new;
mod parse;
mod to_expression;
mod to_static;
mod to_tokens;

pub use kind::ExpressionWithoutBlockKind;

/// An expression that does not have a block

#[derive(Debug, Clone)]
pub struct ExpressionWithoutBlock<'a> {
    /// Attributes affecting this expression
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The kind of expression
    pub kind: ExpressionWithoutBlockKind<'a>,
}
