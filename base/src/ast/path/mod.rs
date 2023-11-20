use super::Punctuated;
use crate::{
    tokens::{DoubleColon, Identifier},
    Parse, ToTokens,
};

/// A path to an item
///
/// Example: `::foo::bar`
#[derive(Clone)]
pub struct Path {
    /// The leading colon
    pub leading: Option<DoubleColon>,

    /// The segments of the path
    pub segments: Punctuated<Identifier, DoubleColon>,
}

impl<'a> Parse<'a> for Path {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        let leading = parser.parse()?;

        let mut segments = Punctuated::new();

        // Push the first element garunteed
        segments.push_element(parser.parse()?);

        while parser.peek::<DoubleColon>() {
            segments.push_seperator(parser.parse()?);
            segments.push_element(parser.parse()?);
        }

        Ok(Path { leading, segments })
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.leading.to_tokens(generator);
        self.segments.to_tokens(generator);
    }
}
