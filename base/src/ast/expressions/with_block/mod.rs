use crate::ast::OuterAttribute;

mod from;
mod kind;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use kind::ExpressionWithBlockKind;

/// An expression that has a block

#[derive(Debug, Clone)]
pub struct ExpressionWithBlock<'a> {
    /// Attributes affecting this expression
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The kind of expression
    pub kind: ExpressionWithBlockKind<'a>,
}
