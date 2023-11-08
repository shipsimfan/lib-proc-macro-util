use crate::{
    ast::Path,
    tokens::{Delimiter, TokenStream},
};

#[derive(Clone)]
pub struct MetaList {
    pub path: Path,
    pub delimiter: Delimiter,
    pub tokens: TokenStream,
}
