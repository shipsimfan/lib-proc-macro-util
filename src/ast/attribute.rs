use crate::{
    ast::Meta,
    parsing::{Parse, Parser},
    tokens::{Bracket, Exclamation, Pound},
    Result, Token,
};

#[derive(Clone, Copy)]
pub enum AttributeTarget {
    Parent(Exclamation),
    Below,
}

#[derive(Clone)]
pub struct Attribute {
    pub pound: Pound,
    pub target: AttributeTarget,
    pub brackets: Bracket,
    pub meta: Meta,
}

impl Attribute {
    pub fn parse_below(parser: &mut Parser) -> Result<Vec<Self>> {
        let mut attributes = Vec::new();
        while parser.peek(Token! [#]) {
            attributes.push(Attribute::parse_single_below(parser)?);
        }
        Ok(attributes)
    }

    fn parse_single_below(parser: &mut Parser) -> Result<Self> {
        let pound = Pound::parse(parser)?;
        let (mut contents, brackets) = Bracket::parse_group(parser)?;

        Ok(Attribute {
            pound,
            target: AttributeTarget::Below,
            brackets,
            meta: contents.parse()?,
        })
    }
}
