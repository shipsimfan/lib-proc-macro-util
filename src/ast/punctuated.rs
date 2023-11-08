pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T, P> Punctuated<T, P> {
    pub const fn new() -> Self {
        Punctuated {
            inner: Vec::new(),
            last: None,
        }
    }

    pub fn empty(&self) -> bool {
        self.inner.len() == 0 && self.last.is_none()
    }

    pub fn trailing_punct(&self) -> bool {
        self.last.is_none() && self.inner.len() > 0
    }

    pub fn push_value(&mut self, value: T) {
        assert!(self.last.is_none());
        self.last = Some(Box::new(value));
    }

    pub fn push_punctuation(&mut self, punctuation: P) {
        assert!(self.last.is_some());
        let last = self.last.take().unwrap();
        self.inner.push((*last, punctuation));
    }
}

impl<T: Clone, P: Clone> Clone for Punctuated<T, P> {
    fn clone(&self) -> Self {
        Punctuated {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}
