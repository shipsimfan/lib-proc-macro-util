use crate::{
    tokens::{Identifier, Type},
    Token,
};

mod value;

mod parse;
mod to_tokens;

pub use value::ConstParamValue;

/// A constant generic parameter
pub struct ConstParam<'a> {
    /// The keyword identifying this parameter as constant
    pub r#const: Token![const],

    /// The name of this parameter
    pub identifier: Identifier,

    /// The colon separating the type
    pub colon: Token![:],

    /// The type of the constant
    pub r#type: Type,

    /// An optional default value for the constant
    pub value: Option<(Token![=], ConstParamValue<'a>)>,
}
