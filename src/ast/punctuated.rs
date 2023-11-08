pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T: Clone, P: Clone> Clone for Punctuated<T, P> {
    fn clone(&self) -> Self {
        Punctuated {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}
