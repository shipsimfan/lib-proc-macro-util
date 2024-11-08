use crate::{
    ast::{GenericArgs, GenericArgsBinding},
    tokens::{Identifier, Type},
    Token,
};

impl<'a> GenericArgsBinding<'a> {
    /// Creates a new [`GenericArgsBinding`]
    pub fn new<I: Into<Identifier>, T: Into<Type>>(
        identifier: I,
        args: Option<GenericArgs<'a>>,
        value: T,
    ) -> Self {
        GenericArgsBinding {
            identifier: identifier.into().into(),
            args: args.map(|args| Box::new(args)),
            equals: Token![=](),
            value: value.into(),
        }
    }
}
