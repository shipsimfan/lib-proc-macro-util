use crate::Token;

mod generic_param;

pub use generic_param::*;

pub struct GenericParams<'a> {
    pub open: Token![<],
    pub params: Vec<(GenericParam<'a>, Token![,])>,
    pub last_param: GenericParam<'a>,
    pub last_comma: Option<Token![,]>,
    pub close: Token![>],
}
