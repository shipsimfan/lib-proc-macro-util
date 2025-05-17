use crate::Token;

mod body;
mod param;
mod parameters;

mod parse;
mod to_static;
mod to_tokens;

pub use body::ClosureBody;
pub use param::ClosureParam;
pub use parameters::ClosureParameters;

/// A closure, or lambda, is a function that can capture variables
#[derive(Debug, Clone)]
pub struct ClosureExpression<'a> {
    /// Indicates if the closure is asynchronous
    pub r#async: Option<Token![async]>,

    /// Indicates if the closure moves captured variables or borrows them
    pub r#move: Option<Token![move]>,

    /// The leading "|" indicating the start of the parameters
    pub leading: Token![|],

    /// The parameters passed into the closure
    pub parameters: Option<ClosureParameters<'a>>,

    /// The trailing "|" indicating the end of the parameters
    pub trailing: Token![|],

    /// The body of the closure
    pub body: ClosureBody<'a>,
}
