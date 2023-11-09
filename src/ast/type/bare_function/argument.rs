use crate::{
    ast::{Attribute, Type},
    tokens::{Colon, Ident},
};

#[derive(Clone)]
pub struct BareFunctionArgument {
    pub attributes: Vec<Attribute>,
    pub name: Option<(Ident, Colon)>,
    pub r#type: Type,
}
