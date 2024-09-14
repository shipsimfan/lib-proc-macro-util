use crate::{
    ast::{SimplePath, SimplePathSegment},
    Token,
};

impl<'a, T: Into<SimplePathSegment<'a>>> From<T> for SimplePath<'a> {
    fn from(value: T) -> Self {
        SimplePath::new_single(value)
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(bool, T)> for SimplePath<'a> {
    fn from(value: (bool, T)) -> Self {
        SimplePath::new_raw(
            if value.0 { Some(Token![::]()) } else { None },
            value.1.into(),
            Vec::new(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Token![::], T)> for SimplePath<'a> {
    fn from(value: (Token![::], T)) -> Self {
        SimplePath::new_raw(Some(value.0), value.1.into(), Vec::new())
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Option<Token![::]>, T)> for SimplePath<'a> {
    fn from(value: (Option<Token![::]>, T)) -> Self {
        SimplePath::new_raw(value.0, value.1.into(), Vec::new())
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<Vec<T>> for SimplePath<'a> {
    fn from(segments: Vec<T>) -> Self {
        SimplePath::new(false, segments)
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(bool, Vec<T>)> for SimplePath<'a> {
    fn from(value: (bool, Vec<T>)) -> Self {
        SimplePath::new(value.0, value.1)
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Token![::], Vec<T>)> for SimplePath<'a> {
    fn from(mut value: (Token![::], Vec<T>)) -> Self {
        SimplePath::new_raw(
            Some(value.0),
            value.1.remove(0).into(),
            value
                .1
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Option<Token![::]>, Vec<T>)> for SimplePath<'a> {
    fn from(mut value: (Option<Token![::]>, Vec<T>)) -> Self {
        SimplePath::new_raw(
            value.0,
            value.1.remove(0).into(),
            value
                .1
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(T, Vec<T>)> for SimplePath<'a> {
    fn from(segments: (T, Vec<T>)) -> Self {
        SimplePath::new_raw(
            None,
            segments.0.into(),
            segments
                .1
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(bool, T, Vec<T>)> for SimplePath<'a> {
    fn from(segments: (bool, T, Vec<T>)) -> Self {
        SimplePath::new_raw(
            if segments.0 { Some(Token![::]()) } else { None },
            segments.1.into(),
            segments
                .2
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Token![::], T, Vec<T>)> for SimplePath<'a> {
    fn from(segments: (Token![::], T, Vec<T>)) -> Self {
        SimplePath::new_raw(
            Some(segments.0),
            segments.1.into(),
            segments
                .2
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}

impl<'a, T: Into<SimplePathSegment<'a>>> From<(Option<Token![::]>, T, Vec<T>)> for SimplePath<'a> {
    fn from(segments: (Option<Token![::]>, T, Vec<T>)) -> Self {
        SimplePath::new_raw(
            segments.0,
            segments.1.into(),
            segments
                .2
                .into_iter()
                .map(|segment| (Token![::](), segment.into()))
                .collect(),
        )
    }
}
