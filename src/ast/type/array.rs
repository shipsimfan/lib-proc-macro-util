use crate::{
    ast::Expression,
    tokens::{Bracket, SemiColon, Type},
};

#[derive(Clone)]
pub struct TypeArray {
    pub bracket: Bracket,
    pub element: Box<Type>,
    pub semi_colon: SemiColon,
    pub length: Expression,
}
