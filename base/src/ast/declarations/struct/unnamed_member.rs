use crate::ast::{Type, Visibility};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::Struct;

/// An unnamed member of a [`Struct`]
#[derive(Clone)]
pub struct UnnamedStructMember<'a> {
    /// The visibility of this member
    pub visibility: Option<Visibility<'a>>,

    /// The type of this member
    pub r#type: Type,
}
