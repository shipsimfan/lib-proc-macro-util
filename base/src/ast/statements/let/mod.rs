use crate::{
    ast::{expressions::BlockExpression, Expression, OuterAttribute, PatternNoTopAlt, Type},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A let statement introduces a new set of variables, given by a pattern.
#[derive(Debug, Clone)]
pub struct LetStatement<'a> {
    /// The attributes affecting this let statement
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The let keyword indicating this is a let statement
    pub r#let: Token![let],

    /// The pattern defining the variable names
    pub pattern: PatternNoTopAlt<'a>,

    /// The type indicator for the variables
    pub r#type: Option<(Token![:], Type<'a>)>,

    /// The expression the variable is being set to
    pub expression: Option<(
        Token![=],
        Expression<'a>,
        Option<(Token![else], BlockExpression<'a>)>,
    )>,
}
