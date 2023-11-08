use crate::{
    ast::{Attribute, Data, Generics, Visibility},
    parsing::{Parse, Parser},
    tokens::Ident,
    Result,
};

#[derive(Clone)]
pub struct DeriveInput {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}

impl<'a> Parse<'a> for DeriveInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = Attribute::parse_below(parser)?;

        todo!()
    }
}
