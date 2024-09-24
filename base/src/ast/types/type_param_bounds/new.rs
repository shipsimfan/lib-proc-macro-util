use crate::{
    ast::{TypeParamBound, TypeParamBounds},
    Token,
};

impl<'a> TypeParamBounds<'a> {
    /// Creates a new set of [`TypeParamBounds`]
    pub fn new(mut bounds: Vec<TypeParamBound<'a>>) -> Self {
        assert!(bounds.len() > 0);
        let first = bounds.remove(0);
        let remaining = bounds
            .into_iter()
            .map(|bound| (Token![+](), bound))
            .collect();

        TypeParamBounds {
            first,
            remaining,
            end: None,
        }
    }
}
