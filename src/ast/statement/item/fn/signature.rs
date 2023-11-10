use crate::{
    ast::{FunctionArgument, Generics, Punctuated, ReturnType, Variadic, ABI},
    tokens::{Async, Comma, Const, Fn, Ident, Parenthesis, Unsafe},
};

#[derive(Clone)]
pub struct Signature {
    pub r#const: Option<Const>,
    pub r#async: Option<Async>,
    pub r#unsafe: Option<Unsafe>,
    pub abi: Option<ABI>,
    pub r#fn: Fn,
    pub ident: Ident,
    pub generics: Generics,
    pub parentheses: Parenthesis,
    pub inputs: Punctuated<FunctionArgument, Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}
