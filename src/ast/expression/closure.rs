use crate::{
    ast::{Attribute, BoundLifetimes, Expression, Pattern, Punctuated, ReturnType},
    tokens::{Async, Comma, Const, Move, Static, VerticalBar},
};

#[derive(Clone)]
pub struct ExpressionClosure {
    pub attributes: Vec<Attribute>,
    pub lifetimes: Option<BoundLifetimes>,
    pub r#const: Option<Const>,
    pub movability: Option<Static>,
    pub r#async: Option<Async>,
    pub capture: Option<Move>,
    pub vertical_bar1: VerticalBar,
    pub inputs: Punctuated<Pattern, Comma>,
    pub vertical_bar2: VerticalBar,
    pub output: ReturnType,
    pub body: Box<Expression>,
}
