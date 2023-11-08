use crate::{
    ast::Attribute,
    parsing::{Parse, Parser},
    Result,
};

#[derive(Clone)]
pub struct DeriveInput {
    pub attributes: Vec<Attribute>,
}

impl<'a> Parse<'a> for DeriveInput {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = Attribute::parse_below(parser)?;

        todo!()
    }
}
