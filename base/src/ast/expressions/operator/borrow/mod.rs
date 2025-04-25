use crate::{ast::Expression, Token};

mod ampersand;

mod parse;
mod to_static;
mod to_tokens;

pub use ampersand::BorrowAmpersand;

/// A borrowing of another expression
#[derive(Debug, Clone)]
pub struct BorrowExpression<'a> {
    /// The leading ampersand(s)
    pub ampersand: BorrowAmpersand,

    /// Is this borrow mutable?
    pub mutable: Option<Token![mut]>,

    /// The expression being borrowed
    pub expression: Box<Expression<'a>>,
}
