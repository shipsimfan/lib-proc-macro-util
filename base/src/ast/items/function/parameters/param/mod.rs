use crate::{
    ast::{OuterAttribute, PatternNoTopAlt, Type},
    Token,
};

#[derive(Debug, Clone)]
pub enum FunctionParam<'a> {
    Variadic {
        attributes: Vec<OuterAttribute<'a>>,
        dots: Token![...],
    },
    Pattern {
        attributes: Vec<OuterAttribute<'a>>,
        pattern: PatternNoTopAlt,
        colon: Token![:],
        r#type: Type<'a>,
    },
}
