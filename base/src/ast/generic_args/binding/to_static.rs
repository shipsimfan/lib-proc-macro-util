use crate::ast::GenericArgsBinding;
use std::borrow::Cow;

impl<'a> GenericArgsBinding<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericArgsBinding<'static> {
        GenericArgsBinding {
            identifier: Cow::Owned(match self.identifier {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            args: self.args.map(|args| Box::new(args.into_static())),
            equals: self.equals,
            value: Box::new(self.value.into_static()),
        }
    }
}
