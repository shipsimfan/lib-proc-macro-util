use crate::ast::Expression;

mod params;

mod to_static;
mod to_tokens;

pub use params::CallParams;

/// A function call
#[derive(Debug, Clone)]
pub struct CallExpression<'a> {
    /// The function being called
    pub function: Box<Expression<'a>>,

    /// The parameters to the function
    pub parameters: CallParams<'a>,
}
