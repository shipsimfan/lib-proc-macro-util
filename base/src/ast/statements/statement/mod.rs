use crate::ast::{
    statements::{ExpressionStatement, LetStatement},
    Item, MacroInvocationSemi,
};

mod parse;
mod to_static;
mod to_tokens;

/// A statement is a component of a block, which is in turn a component of an outer expression or
/// function.
#[derive(Debug, Clone)]
pub enum Statement<'a> {
    /// A definition of an item
    Item(Item<'a>),

    /// A statement introducing new variables
    Let(LetStatement<'a>),

    /// An expression whose results are ignored
    Expression(ExpressionStatement<'a>),

    /// An macro invokation that may not have a semi colon after it
    MacroInvocation(MacroInvocationSemi<'a>),
}
