use super::Punctuated;
use crate::Token;

mod generic;
mod lifetime;
mod r#type;

pub use generic::Generic;
pub use lifetime::*;
pub use r#type::*;

/// A list of [`Generic`]s
#[derive(Clone)]
pub struct Generics {
    /// The start of the generic arguments
    pub left_triangle: Token![<],

    /// The generic arguments themselves
    pub arguments: Punctuated<Generic, Token![,]>,

    /// The end of the generic arguments
    pub right_triangle: Token![>],
}
