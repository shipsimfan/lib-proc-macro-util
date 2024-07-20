use crate::{
    ast::{GenericTypeConstraint, Punctuated},
    Token,
};

/// A list of constraints on a generic type
#[derive(Clone)]
pub struct GenericTypeConstraints {
    /// The colon starting the constraints
    pub colon: Token![:],

    /// The constraints on the type
    pub constraints: Punctuated<GenericTypeConstraint, Token![+]>,
}
