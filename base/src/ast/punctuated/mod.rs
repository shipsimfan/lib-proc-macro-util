use crate::{Parse, Parser, Result, ToTokens};
use std::collections::VecDeque;

mod iter;

pub use iter::{IntoIter, Iter};

/// A series of `Element`s punctuated by `Separator`s
#[derive(Debug, Clone)]
pub struct Punctuated<Element, Separator> {
    inner: VecDeque<(Element, Separator)>,
    last: Option<Box<Element>>,
}

impl<Element, Separator> Punctuated<Element, Separator> {
    /// Creates a new empty [`Punctuated`]
    ///
    ///
    /// ## Return Value
    /// Returns the newly created empty [`Punctuated`]
    pub const fn new() -> Self {
        Punctuated {
            inner: VecDeque::new(),
            last: None,
        }
    }

    /// Gets the number of elements in this list
    pub fn len(&self) -> usize {
        self.inner.len() + if self.last.is_some() { 1 } else { 0 }
    }

    /// Is the last element in this list a separator?
    pub fn has_final_separator(&self) -> bool {
        self.last.is_none()
    }

    /// Creates a borrowed iterator over the elements
    pub fn iter(&self) -> Iter<Element, Separator> {
        Iter::new(
            self.inner.iter(),
            self.last.as_ref().map(|last| last.as_ref()),
        )
    }

    /// Pushes an element to the end
    ///
    /// ## Parameters
    ///  * `element` - The element to be pushed
    ///
    /// ## Remarks
    /// This function will panic if the sequence doesn't have a separator preceding this element.
    pub fn push_element(&mut self, element: Element) {
        assert!(self.last.is_none());
        self.last = Some(Box::new(element));
    }

    /// Pushes a separator to the end
    ///
    /// ## Parameters
    ///  * `separator` - The separator to be pushed
    ///
    /// ## Remarks
    /// This function will panic if the sequence doesn't have an element preceding this separator.
    pub fn push_separator(&mut self, separator: Separator) {
        assert!(self.last.is_some());
        self.inner
            .push_back((*self.last.take().unwrap(), separator));
    }

    /// Pushes an element onto the front of the list
    pub fn push_front(&mut self, element: Element, separator: Separator) {
        self.inner.push_front((element, separator));
    }
}

impl<'a, Element: Parse<'a>, Separator: Parse<'a>> Punctuated<Element, Separator> {
    /// Parses a [`Punctuated`] list from `parser`
    pub fn parse(
        parser: &mut Parser<'a>,
        first_required: bool,
        optional_final_separator: bool,
    ) -> Result<Self> {
        let mut inner = Punctuated::new();

        if !first_required && !parser.peek::<Element>() {
            return Ok(inner);
        }
        inner.push_element(parser.parse()?);

        while parser.peek::<Separator>() {
            inner.push_separator(parser.parse()?);

            if optional_final_separator && !parser.peek::<Element>() {
                break;
            }

            inner.push_element(parser.parse()?);
        }

        Ok(inner)
    }
}

impl<Element: ToTokens, Separator: ToTokens> ToTokens for Punctuated<Element, Separator> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        for (element, separator) in &self.inner {
            element.to_tokens(generator);
            separator.to_tokens(generator);
        }

        self.last.to_tokens(generator);
    }
}

impl<Element, Separator> IntoIterator for Punctuated<Element, Separator> {
    type IntoIter = IntoIter<Element, Separator>;
    type Item = (Element, Option<Separator>);

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.inner.into_iter(), self.last)
    }
}

impl<'a, Element, Separator> IntoIterator for &'a Punctuated<Element, Separator> {
    type IntoIter = Iter<'a, Element, Separator>;
    type Item = (&'a Element, Option<&'a Separator>);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
