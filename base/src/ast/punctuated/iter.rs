pub struct IntoIter<Element, Seperator> {
    iter: std::vec::IntoIter<(Element, Seperator)>,
    last: Option<Box<Element>>,
}

pub struct Iter<'a, Element, Seperator> {
    iter: std::slice::Iter<'a, (Element, Seperator)>,
    last: Option<&'a Element>,
}

impl<Element, Seperator> IntoIter<Element, Seperator> {
    pub const fn new(
        iter: std::vec::IntoIter<(Element, Seperator)>,
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
    pub const fn new(
        iter: std::slice::Iter<'a, (Element, Seperator)>,
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
