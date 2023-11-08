use crate::{
    ast::Path,
    parsing::{Parse, Parser},
    Result,
};

mod list;
mod name_value;

pub use list::MetaList;
pub use name_value::MetaNameValue;

#[derive(Clone)]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

impl<'a> Parse<'a> for Meta {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
