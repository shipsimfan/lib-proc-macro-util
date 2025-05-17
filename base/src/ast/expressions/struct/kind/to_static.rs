use crate::ast::expressions::{StructExprKind, StructExprStruct, StructExprTuple};

impl<'a> StructExprKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExprKind<'static> {
        match self {
            StructExprKind::Struct(r#struct) => {
                StructExprKind::Struct(r#struct.map(StructExprStruct::into_static))
            }
            StructExprKind::Tuple(tuple) => {
                StructExprKind::Tuple(tuple.map(StructExprTuple::into_static))
            }
            StructExprKind::Unit => StructExprKind::Unit,
        }
    }
}
