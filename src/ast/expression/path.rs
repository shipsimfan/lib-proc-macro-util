use crate::ast::{Attribute, Path, QSelf};

#[derive(Clone)]
pub struct ExpressionPath {
    pub attributes: Vec<Attribute>,
    pub q_self: Option<QSelf>,
    pub path: Path,
}
