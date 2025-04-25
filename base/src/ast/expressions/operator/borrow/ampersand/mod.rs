use crate::Token;

mod parse;
mod to_tokens;

/// The leading ampersand(s) for a [`BorrowExpression`](crate::ast::expressions::BorrowExpression)
#[derive(Debug, Clone)]
pub enum BorrowAmpersand {
    /// There is only one ampersand before
    Single(Token![&]),

    /// There are two ampersands before
    Double(Token![&&]),
}
