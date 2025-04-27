use crate::{
    ast::{patterns::RestPattern, Pattern},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// The items which make up a tuple pattern
#[derive(Debug, Clone)]
pub enum TuplePatternItems<'a> {
    /// The tuple only has a single pattern
    Single(Box<Pattern<'a>>, Token![,]),

    /// The tuple only contains a rest pattern
    Rest(RestPattern),

    /// The tuple contains multiple patterns
    Multiple {
        /// The first pattern in the tuple
        first: Box<Pattern<'a>>,

        /// The comma seperating the first and second pattern
        first_comma: Token![,],

        /// The second pattern in the tuple
        second: Box<Pattern<'a>>,

        /// The remaining patterns in the tuple and their separators
        remaining: Vec<(Token![,], Pattern<'a>)>,

        /// A last optional comma
        last: Option<Token![,]>,
    },
}
