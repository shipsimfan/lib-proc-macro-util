use crate::{ast::OuterAttribute, Token};

mod param;

pub use param::{MaybeNamedParam, MaybeNamedParamName};

#[derive(Debug, Clone)]
pub struct MaybeNamedFunctionParameters<'a> {
    pub first: MaybeNamedParam<'a>,
    pub remaining: Vec<(Token![,], MaybeNamedParam<'a>)>,
    pub variadic: Vec<(Token![,], Vec<OuterAttribute<'a>>, Token![...])>,
    pub ending: Token![,],
}
