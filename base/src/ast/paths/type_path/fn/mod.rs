use crate::{ast::TypeNoBounds, Token};

mod inputs;

pub use inputs::TypePathFnInputs;

pub struct TypePathFn {
    pub inputs: Option<TypePathFnInputs>,
    pub bounds: Option<(Token![->], TypeNoBounds)>,
}
