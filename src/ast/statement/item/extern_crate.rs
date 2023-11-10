use crate::{
    ast::{Attribute, Visibility},
    tokens::{As, Crate, Extern, Ident, SemiColon},
};

#[derive(Clone)]
pub struct ItemExternCrate {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#extern: Extern,
    pub krate: Crate,
    pub ident: Ident,
    pub rename: Option<(As, Ident)>,
    pub semi_colon: SemiColon,
}
