use crate::{ast::SimplePathSegment, Generator, ToTokens};

impl<'a> ToTokens for SimplePathSegment<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            SimplePathSegment::Identifier(identifier) => identifier.to_tokens(generator),
            SimplePathSegment::Crate(krate) => krate.to_tokens(generator),
            SimplePathSegment::_Self(_self) => _self.to_tokens(generator),
            SimplePathSegment::Super(_super) => _super.to_tokens(generator),
            Self::DollarCrate(dollar, krate) => {
                dollar.to_tokens(generator);
                krate.to_tokens(generator);
            }
        }
    }
}
