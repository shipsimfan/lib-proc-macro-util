use crate::ast::items::VisItemKind;

impl<'a> VisItemKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> VisItemKind<'static> {
        match self {
            VisItemKind::Module(module) => VisItemKind::Module(module.into_static()),
            VisItemKind::ExternCrate(r#extern) => VisItemKind::ExternCrate(r#extern.into_static()),
            VisItemKind::Use(r#use) => VisItemKind::Use(r#use.into_static()),
            VisItemKind::Function(function) => VisItemKind::Function(function.into_static()),
            VisItemKind::Struct(r#struct) => VisItemKind::Struct(r#struct.into_static()),
            VisItemKind::Enumeration(r#enum) => VisItemKind::Enumeration(r#enum.into_static()),
        }
    }
}
