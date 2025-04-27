use crate::{
    ast::patterns::{StructPatternEtCetera, StructPatternFields},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// The elements which make up a struct pattern
#[derive(Debug, Clone)]
pub enum StructPatternElements<'a> {
    /// The struct pattern has fields
    Fields(
        StructPatternFields<'a>,
        Option<(Token![,], Option<StructPatternEtCetera<'a>>)>,
    ),

    /// The struct pattern only has remaining
    EtCetera(StructPatternEtCetera<'a>),
}
