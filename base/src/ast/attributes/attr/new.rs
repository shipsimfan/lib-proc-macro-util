use crate::ast::{Attr, AttrInput, SimplePath};

impl<'a> Attr<'a> {
    /// Creates a new [`Attr`] from `path` and `input`
    pub const fn new_raw(path: SimplePath<'a>, input: Option<AttrInput<'a>>) -> Self {
        Attr { path, input }
    }

    /// Creates a new [`Attr`] from `path` with no input
    pub fn new<T: Into<SimplePath<'a>>>(path: T) -> Self {
        Attr::new_raw(path.into(), None)
    }

    /// Creates a new [`Attr`] from `path` and `input`
    pub fn new_input<T1: Into<SimplePath<'a>>, T2: Into<AttrInput<'a>>>(
        path: T1,
        input: T2,
    ) -> Self {
        Attr::new_raw(path.into(), Some(input.into()))
    }
}
