use crate::{
    ast::{Attribute, Lifetime, Type},
    tokens::{Ampersand, Colon, Mut, SelfValue},
};

#[derive(Clone)]
pub struct Receiver {
    pub attributes: Vec<Attribute>,
    pub reference: Option<(Ampersand, Option<Lifetime>)>,
    pub mutability: Option<Mut>,
    pub self_value: SelfValue,
    pub colon: Option<Colon>,
    pub r#type: Box<Type>,
}
