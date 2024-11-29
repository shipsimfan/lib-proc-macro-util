use crate::ast::{Attr, AttrInput};

impl<'a> Attr<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Attr<'static> {
        Attr {
            path: self.path.into_static(),
            input: self.input.map(AttrInput::into_static),
        }
    }
}
