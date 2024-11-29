use crate::{
    ast::{GenericArgs, GenericArgsBinding, Type},
    tokens::Identifier,
    Token,
};

impl<'a> GenericArgsBinding<'a> {
    /// Creates a new [`GenericArgsBinding`]
    pub fn new<I: Into<Identifier>, T: Into<Type<'a>>>(
        identifier: I,
        args: Option<GenericArgs<'a>>,
        value: T,
    ) -> Self {
        GenericArgsBinding {
            identifier: identifier.into().into(),
            args: args.map(|args| Box::new(args)),
            equals: Token![=](),
            value: Box::new(value.into()),
        }
    }
}
