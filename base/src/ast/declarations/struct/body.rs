use crate::{
    ast::{NamedStructMember, Punctuated, UnnamedStructMember},
    Token,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::Struct;

/// The body of a [`Struct`]
#[derive(Clone)]
pub enum StructBody<'a> {
    /// The struct has no body
    None(Token![;]),

    /// The struct constists of unnamed members
    Unnamed(Punctuated<UnnamedStructMember<'a>, Token![,]>),

    /// The struct consists of named members
    Named(Punctuated<NamedStructMember<'a>, Token![,]>),
}
