use crate::ast::items::{EnumItemKind, StructFields, TupleFields};

impl<'a> EnumItemKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> EnumItemKind<'static> {
        match self {
            EnumItemKind::Tuple(tuple) => EnumItemKind::Tuple(tuple.map(TupleFields::into_static)),
            EnumItemKind::Struct(r#struct) => {
                EnumItemKind::Struct(r#struct.map(StructFields::into_static))
            }
        }
    }
}
