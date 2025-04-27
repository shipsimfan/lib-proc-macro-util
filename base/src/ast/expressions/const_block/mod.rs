use crate::{ast::expressions::BlockExpression, Token};

mod default;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// An constant block of code
#[derive(Debug, Clone)]
pub struct ConstBlockExpression<'a> {
    /// The const identifying this expression
    pub r#const: Token![const],

    /// The block of constant code
    pub block: BlockExpression<'a>,
}
