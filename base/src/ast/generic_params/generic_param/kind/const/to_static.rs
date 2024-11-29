use crate::ast::ConstParam;
use std::borrow::Cow;

impl<'a> ConstParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ConstParam<'static> {
        ConstParam {
            r#const: self.r#const,
            identifier: Cow::Owned(match self.identifier {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            colon: self.colon,
            r#type: Box::new(self.r#type.into_static()),
            value: self.value.map(|(eq, param)| (eq, param.into_static())),
        }
    }
}
