use crate::tokens::{Extern, Literal};

#[derive(Clone)]
pub struct ABI {
    pub r#extern: Extern,
    pub name: Option<Literal>,
}
