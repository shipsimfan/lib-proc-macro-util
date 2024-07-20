use crate::{
    ast::{Lifetime, Type},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A single generic argument
#[derive(Clone)]
pub enum GenericArgument {
    /// A regular type
    Type(Type),

    /// A trait object
    Trait {
        /// The `dyn` keyword identifying this construct
        r#dyn: Token![dyn],

        /// The type of the trait object
        r#type: Type,
    },

    /// A lifetime
    Lifetime(Lifetime),
}

impl<'a> Parse<'a> for GenericArgument {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token!['_]>() {
            return parser
                .parse()
                .map(|lifetime| GenericArgument::Lifetime(lifetime));
        }

        let r#dyn: Option<Token![dyn]> = parser.parse()?;
        let r#type = parser.parse()?;

        Ok(match r#dyn {
            Some(r#dyn) => GenericArgument::Trait { r#dyn, r#type },
            None => GenericArgument::Type(r#type),
        })
    }
}

impl ToTokens for GenericArgument {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            GenericArgument::Type(r#type) => r#type.to_tokens(generator),
            GenericArgument::Trait { r#dyn, r#type } => {
                r#dyn.to_tokens(generator);
                r#type.to_tokens(generator);
            }
            GenericArgument::Lifetime(lifetime) => lifetime.to_tokens(generator),
        }
    }
}
