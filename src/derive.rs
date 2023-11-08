use crate::{
    ast::Attribute,
    parsing::{Parse, Parser},
    Result,
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DeriveInput {
    pub attributes: Vec<Attribute>,
}

impl Parse for DeriveInput {
    fn parse(parser: &mut Parser) -> Result<Self> {
        let attributes = Attribute::parse_below(parser)?;

        todo!()
    }
}
