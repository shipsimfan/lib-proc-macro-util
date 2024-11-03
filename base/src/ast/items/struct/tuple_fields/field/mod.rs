use crate::ast::{OuterAttribute, Type, Visibility};

#[derive(Debug, Clone)]
pub struct TupleField<'a> {
    pub attributes: Vec<OuterAttribute<'a>>,
    pub visibility: Option<Visibility<'a>>,
    pub r#type: Type<'a>,
}
