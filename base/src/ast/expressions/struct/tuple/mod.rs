use crate::{
    ast::{Expression, PathInExpression},
    Token,
};

/// A struct expression which is of a tuple-style
#[derive(Debug, Clone)]
pub struct StructExprTuple<'a> {
    /// The path to the struct
    pub path: PathInExpression<'a>,

    /// The expressions which make up the tuple
    pub elements: Option<(
        Box<Expression<'a>>,
        Vec<(Token![,], Expression<'a>)>,
        Option<Token![,]>,
    )>,
}
