use crate::{
    ast::{OuterAttribute, PatternNoTopAlt, Type},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A function parameter
#[derive(Debug, Clone)]
pub enum FunctionParam<'a> {
    /// The function parameter is variadic
    Variadic {
        /// Attributes affecting this parameter
        attributes: Vec<OuterAttribute<'a>>,

        /// The `...` indicating this is variadic
        dots: Token![...],
    },

    /// The function parameter is a normal parameter
    Pattern {
        /// Attributes affecting this parameter
        attributes: Vec<OuterAttribute<'a>>,

        /// The pattern giving the parameter name
        pattern: PatternNoTopAlt<'a>,

        /// The colon separating the pattern from the type
        colon: Token![:],

        /// The type of the parameter
        r#type: Type<'a>,
    },
}
