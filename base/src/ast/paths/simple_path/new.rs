use crate::{
    ast::{SimplePath, SimplePathSegment},
    Token,
};

impl<'a> SimplePath<'a> {
    /// Creates a new [`SimplePath`]
    pub const fn new_raw(
        leading: Option<Token![::]>,
        first: SimplePathSegment<'a>,
        remaining: Vec<(Token![::], SimplePathSegment<'a>)>,
    ) -> Self {
        SimplePath {
            leading,
            first,
            remaining,
        }
    }

    /// Creates a new [`SimplePath`]
    pub fn new<T: Into<SimplePathSegment<'a>>>(leading: bool, mut segments: Vec<T>) -> Self {
        assert!(
            segments.len() > 0,
            "cannot create a simple path with no segments"
        );
        SimplePath::new_raw(
            if leading { Some(Token![::]()) } else { None },
            segments.remove(0).into(),
            segments
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }

    /// Creates a new [`SimplePath`] with only a single segment
    pub fn new_single<T: Into<SimplePathSegment<'a>>>(value: T) -> Self {
        SimplePath::new_raw(None, value.into(), Vec::new())
    }
}
