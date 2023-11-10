use crate::tokens::{As, Ident};

#[derive(Clone)]
pub struct UseRename {
    pub ident: Ident,
    pub r#as: As,
    pub rename: Ident,
}
