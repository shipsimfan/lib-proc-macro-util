use crate::{
    tokens::{Identifier, Type},
    Token,
};

mod value;

pub use value::ConstParamValue;

pub struct ConstParam<'a> {
    pub r#const: Token![const],
    pub identifier: Identifier,
    pub colon: Token![:],
    pub r#type: Type,
    pub value: Option<(Token![=], ConstParamValue<'a>)>,
}
