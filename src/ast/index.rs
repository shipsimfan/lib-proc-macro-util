use crate::tokens::Span;

#[derive(Clone)]
pub struct Index {
    pub index: u32,
    pub span: Span,
}
