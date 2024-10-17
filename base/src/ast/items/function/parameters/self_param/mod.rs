use crate::{
    ast::{Lifetime, OuterAttribute, Type},
    Token,
};

#[derive(Debug, Clone)]
pub struct SelfParam<'a> {
    pub attributes: Vec<OuterAttribute<'a>>,
    pub reference: Option<(Token![&], Option<Lifetime<'a>>)>,
    pub r#mut: Option<Token![mut]>,
    pub _self: Token![self],
    pub r#type: Option<(Token![:], Type<'a>)>,
}
