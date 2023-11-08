use crate::{
    ast::{GenericParameter, Punctuated},
    tokens::{Comma, For, LeftTriangle, RightTriangle},
};

#[derive(Clone)]
pub struct BoundLifetimes {
    pub r#for: For,
    pub left_triangle: LeftTriangle,
    pub lifetimes: Punctuated<GenericParameter, Comma>,
    pub right_triangle: RightTriangle,
}
