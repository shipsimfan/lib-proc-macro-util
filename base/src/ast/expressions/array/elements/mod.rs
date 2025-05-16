use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The elements of an array
#[derive(Debug, Clone)]
pub enum ArrayElements<'a> {
    /// Each element of the array is defined
    Every {
        /// The first defined element
        first: Box<Expression<'a>>,

        /// The remaining elements and their separators
        remaining: Vec<(Token![,], Expression<'a>)>,

        /// An optional final comma
        last: Option<Token![,]>,
    },

    /// The elements of the array are uniform
    Uniform {
        /// The element to clone into the array
        element: Box<Expression<'a>>,

        /// A semi-colon separating the element and count
        semi: Token![;],

        /// The number of elements long to make the array
        count: Box<Expression<'a>>,
    },
}
