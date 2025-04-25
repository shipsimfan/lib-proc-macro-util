use crate::{ast::expressions::BlockExpression, Token};

mod default;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// An unsafe block of code
#[derive(Debug, Clone)]
pub struct UnsafeBlockExpression<'a> {
    /// The unsafe identifying this expression
    pub r#unsafe: Token![unsafe],

    /// The block of unsafe code
    pub block: BlockExpression<'a>,
}
