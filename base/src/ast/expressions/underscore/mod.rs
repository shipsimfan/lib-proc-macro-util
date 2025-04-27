use crate::Token;

mod default;
mod new;
mod parse;
mod to_tokens;

/// Underscore expressions, denoted with the symbol _, are used to signify a placeholder in a
/// destructuring assignment.
#[derive(Debug, Clone)]
pub struct UnderscoreExpression {
    /// The underscore token itself
    pub underscore: Token![_],
}
