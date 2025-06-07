use crate::{ast::PathIdentSegment, Generator, ToTokens};

impl<'a> ToTokens for PathIdentSegment<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            PathIdentSegment::Identifier(identifier) => identifier.to_tokens(generator),
            PathIdentSegment::Super(_super) => _super.to_tokens(generator),
            PathIdentSegment::_LowerSelf(_self) => _self.to_tokens(generator),
            PathIdentSegment::_UpperSelf(_self) => _self.to_tokens(generator),
            PathIdentSegment::_Crate(_crate) => _crate.to_tokens(generator),
            PathIdentSegment::DollarCrate(dollar, _crate) => {
                dollar.to_tokens(generator);
                _crate.to_tokens(generator);
            }
        }
    }
}
