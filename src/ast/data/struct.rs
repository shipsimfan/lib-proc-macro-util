use crate::{
    ast::Fields,
    tokens::{SemiColon, Struct},
};

#[derive(Clone)]
pub struct DataStruct {
    pub r#struct: Struct,
    pub fields: Fields,
    pub semi_colon: Option<SemiColon>,
}
