use crate::{
    ast::{GenericArgument, PathSeperator, Punctuated},
    tokens::{Comma, LeftTriangle, RightTriangle},
};

#[derive(Clone)]
pub struct AngleBracketGenerics {
    pub double_colon: Option<PathSeperator>,
    pub left_triangle: LeftTriangle,
    pub args: Punctuated<GenericArgument, Comma>,
    pub right_trangle: RightTriangle,
}
