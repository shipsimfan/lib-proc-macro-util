use crate::ast::{UseGlob, UseGroup, UseName, UsePath, UseRename};

#[derive(Clone)]
pub enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}
