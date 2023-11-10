use crate::{
    ast::{Attribute, Signature, Visibility},
    tokens::SemiColon,
};

#[derive(Clone)]
pub struct ForeignItemFunction {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub semi_colon: SemiColon,
}
