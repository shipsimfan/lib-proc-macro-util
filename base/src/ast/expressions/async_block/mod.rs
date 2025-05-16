use crate::{ast::expressions::BlockExpression, Token};

mod default;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// An asynchronous block of code which evaluates to a future
#[derive(Debug, Clone)]
pub struct AsyncBlockExpression<'a> {
    /// The async identifying this expression
    pub r#async: Token![async],

    /// An optional keyword indicating enclosed variables should moved instead of borrowed
    pub r#move: Option<Token![move]>,

    /// The block of asynchronous code
    pub block: BlockExpression<'a>,
}
