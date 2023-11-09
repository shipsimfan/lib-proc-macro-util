use crate::{
    ast::{Attribute, Expression, FieldValue, Path, Punctuated, QSelf},
    tokens::{Brace, Comma, DoubleDot},
};

#[derive(Clone)]
pub struct ExpressionStruct {
    pub attributes: Vec<Attribute>,
    pub q_self: Option<QSelf>,
    pub path: Path,
    pub brace: Brace,
    pub fields: Punctuated<FieldValue, Comma>,
    pub double_dot: Option<DoubleDot>,
    pub rest: Option<Box<Expression>>,
}
