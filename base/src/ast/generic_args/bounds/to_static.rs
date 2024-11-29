use crate::ast::GenericArgsBounds;
use std::borrow::Cow;

impl<'a> GenericArgsBounds<'a> {
    pub fn into_static(self) -> GenericArgsBounds<'static> {
        GenericArgsBounds {
            identifier: Cow::Owned(match self.identifier {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            args: self.args.map(|args| Box::new(args.into_static())),
            colon: self.colon,
            bounds: self.bounds.into_static(),
        }
    }
}
