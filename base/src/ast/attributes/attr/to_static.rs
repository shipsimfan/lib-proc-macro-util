use crate::ast::{Attr, AttrInput};

impl<'a> Attr<'a> {
    pub fn into_static(self) -> Attr<'static> {
        Attr {
            path: self.path.into_static(),
            input: self.input.map(AttrInput::into_static),
        }
    }
}
