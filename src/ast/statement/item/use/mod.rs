use crate::{
    ast::{Attribute, Visibility},
    tokens::{DoubleColon, SemiColon, Use},
};

mod glob;
mod group;
mod name;
mod path;
mod rename;
mod tree;

pub use glob::UseGlob;
pub use group::UseGroup;
pub use name::UseName;
pub use path::UsePath;
pub use rename::UseRename;
pub use tree::UseTree;

#[derive(Clone)]
pub struct ItemUse {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#use: Use,
    pub leading_colon: Option<DoubleColon>,
    pub tree: UseTree,
    pub semi_colon: SemiColon,
}
