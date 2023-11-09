use crate::ast::{Path, QSelf};

#[derive(Clone)]
pub struct TypePath {
    pub qself: Option<QSelf>,
    pub path: Path,
}
