// rustdoc imports
#[allow(unused_imports)]
use super::Punctuated;

/// An owning iterator over the elements of a [`Punctuated`]
pub struct IntoIter<Element, Seperator> {
    iter: std::collections::vec_deque::IntoIter<(Element, Seperator)>,
    last: Option<Box<Element>>,
}

/// An iterator over thelements of a [`Punctuated`]
pub struct Iter<'a, Element, Seperator> {
    iter: std::collections::vec_deque::Iter<'a, (Element, Seperator)>,
    last: Option<&'a Element>,
}

/// A mutable iterator over thelements of a [`Punctuated`]
pub struct IterMut<'a, Element, Seperator> {
    iter: std::collections::vec_deque::IterMut<'a, (Element, Seperator)>,
    last: Option<&'a mut Element>,
}

impl<Element, Seperator> IntoIter<Element, Seperator> {
    pub(super) const fn new(
        iter: std::collections::vec_deque::IntoIter<(Element, Seperator)>,
        last: Option<Box<Element>>,
    ) -> Self {
        IntoIter { iter, last }
    }
}

impl<Element, Seperator> Iterator for IntoIter<Element, Seperator> {
    type Item = (Element, Option<Seperator>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((element, seperator)) = self.iter.next() {
            return Some((element, Some(seperator)));
        }

        self.last.take().map(|element| (*element, None))
    }
}

impl<'a, Element, Seperator> Iter<'a, Element, Seperator> {
    pub(super) const fn new(
        iter: std::collections::vec_deque::Iter<'a, (Element, Seperator)>,
        last: Option<&'a Element>,
    ) -> Self {
        Iter { iter, last }
    }
}

impl<'a, Element, Seperator> Iterator for Iter<'a, Element, Seperator> {
    type Item = (&'a Element, Option<&'a Seperator>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((element, seperator)) = self.iter.next() {
            return Some((element, Some(seperator)));
        }

        self.last.take().map(|element| (element, None))
    }
}

impl<'a, Element, Seperator> IterMut<'a, Element, Seperator> {
    pub(super) fn new(
        iter: std::collections::vec_deque::IterMut<'a, (Element, Seperator)>,
        last: Option<&'a mut Element>,
    ) -> Self {
        IterMut { iter, last }
    }
}

impl<'a, Element, Seperator> Iterator for IterMut<'a, Element, Seperator> {
    type Item = (&'a mut Element, Option<&'a mut Seperator>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((element, seperator)) = self.iter.next() {
            return Some((element, Some(seperator)));
        }

        self.last.take().map(|element| (element, None))
    }
}
