use crate::{
    ast::{BoundLifetimes, Punctuated, ReturnType, ABI},
    tokens::{Comma, Fn, Parenthesis, Unsafe},
};

mod argument;
mod variadic;

pub use argument::BareFunctionArgument;
pub use variadic::BareVariadic;

#[derive(Clone)]
pub struct TypeBareFunction {
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Option<Unsafe>,
    pub abi: Option<ABI>,
    pub r#fn: Fn,
    pub parentheses: Parenthesis,
    pub inputs: Punctuated<BareFunctionArgument, Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
