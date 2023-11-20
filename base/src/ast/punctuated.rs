use crate::ToTokens;

/// A series of `Element`s punctuated by `Seperator`s
#[derive(Clone)]
pub struct Punctuated<Element, Seperator> {
    inner: Vec<(Element, Seperator)>,
    last: Option<Box<Element>>,
}

impl<Element, Seperator> Punctuated<Element, Seperator> {
    /// Creates a new empty [`Punctuated`]
    ///
    /// ## Return Value
    /// Returns the newly created empty [`Punctuated`]
    pub const fn new() -> Self {
        Punctuated {
            inner: Vec::new(),
            last: None,
        }
    }

    /// Pushes an element to the end
    ///
    /// ## Parameters
    ///  * `element` - The element to be pushed
    ///
    /// ## Remarks
    /// This function will panic if the sequence doesn't have a seperator preceding this element.
    pub fn push_element(&mut self, element: Element) {
        assert!(self.last.is_none());
        self.last = Some(Box::new(element));
    }

    /// Pushes a seperator to the end
    ///
    /// ## Parameters
    ///  * `seperator` - The seperator to be pushed
    ///
    /// ## Remarks
    /// This function will panic if the sequence doesn't have an element preceding this seperator.
    pub fn push_seperator(&mut self, seperator: Seperator) {
        assert!(self.last.is_some());
        self.inner.push((*self.last.take().unwrap(), seperator));
    }
}

impl<Element: ToTokens, Seperator: ToTokens> ToTokens for Punctuated<Element, Seperator> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        for (element, seperator) in &self.inner {
            element.to_tokens(generator);
            seperator.to_tokens(generator);
        }

        self.last.to_tokens(generator);
    }
}
