use crate::ast::OuterAttribute;

mod kind;

pub use kind::*;

pub struct GenericParam<'a> {
    pub attributes: Vec<OuterAttribute<'a>>,
    pub kind: GenericParamKind<'a>,
}
