use crate::{ast::Type, tokens::Identifier, Token};
use std::borrow::Cow;

mod value;

mod parse;
mod to_generic_arg;
mod to_static;
mod to_tokens;

pub use value::ConstParamValue;

/// A constant generic parameter
#[derive(Debug, Clone)]
pub struct ConstParam<'a> {
    /// The keyword identifying this parameter as constant
    pub r#const: Token![const],

    /// The name of this parameter
    pub identifier: Cow<'a, Identifier>,

    /// The colon separating the type
    pub colon: Token![:],

    /// The type of the constant
    pub r#type: Box<Type<'a>>,

    /// An optional default value for the constant
    pub value: Option<(Token![=], ConstParamValue<'a>)>,
}
