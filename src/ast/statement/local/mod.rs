use crate::{
    ast::{Attribute, Pattern},
    tokens::{Let, SemiColon},
};

mod init;

pub use init::LocalInit;

#[derive(Clone)]
pub struct Local {
    pub attributes: Vec<Attribute>,
    pub r#let: Let,
    pub pattern: Pattern,
    pub init: Option<LocalInit>,
    pub semi_colon: SemiColon,
}
