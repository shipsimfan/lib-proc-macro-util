use crate::tokens::{Asterick, Const, Mut, Type};

#[derive(Clone)]
pub struct TypePointer {
    pub asterick: Asterick,
    pub r#const: Option<Const>,
    pub mutability: Option<Mut>,
    pub element: Box<Type>,
}
