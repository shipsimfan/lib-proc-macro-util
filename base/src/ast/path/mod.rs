use crate::{ast::Punctuated, tokens::Identifier, Parse, ToTokens, Token};

mod generic_argument;
mod generic_arguments;
mod segment;

pub use generic_argument::GenericArgument;
pub use generic_arguments::GenericArguments;
pub use segment::PathSegment;

/// A path to an item
///
/// Example: `::foo::bar`
#[derive(Clone)]
pub struct Path {
    /// The leading colon
    pub leading: Option<Token![::]>,

    /// The segments of the path
    pub segments: Punctuated<PathSegment, Token![::]>,
}

impl<'a> Parse<'a> for Path {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        let leading = parser.parse()?;
        let segments = Punctuated::parse(parser, true, false)?;

        Ok(Path { leading, segments })
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.leading.to_tokens(generator);
        self.segments.to_tokens(generator);
    }
}

impl From<Identifier> for Path {
    fn from(identifier: Identifier) -> Self {
        let mut segments = Punctuated::new();
        segments.push_element(identifier.into());

        Path {
            leading: None,
            segments,
        }
    }
}
