use crate::ast::{items::ModuleBody, InnerAttribute, Item};

impl<'a> ModuleBody<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ModuleBody<'static> {
        match self {
            ModuleBody::None(semi) => ModuleBody::None(semi),
            ModuleBody::Some { attributes, items } => ModuleBody::Some {
                attributes: attributes
                    .into_iter()
                    .map(InnerAttribute::into_static)
                    .collect(),
                items: items.into_iter().map(Item::into_static).collect(),
            },
        }
    }
}
