use crate::tokens::{Bracket, Type};

#[derive(Clone)]
pub struct TypeSlice {
    pub bracket: Bracket,
    pub element: Box<Type>,
}
