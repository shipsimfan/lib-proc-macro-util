use crate::tokens::Span;

#[derive(Clone)]
pub struct PathSeperator {
    pub spans: [Span; 2],
}
