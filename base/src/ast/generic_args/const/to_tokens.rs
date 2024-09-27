use crate::{ast::GenericArgsConst, Generator, ToTokens};

impl<'a> ToTokens for GenericArgsConst<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            GenericArgsConst::Block(block) => block.to_tokens(generator),
            GenericArgsConst::Literal(literal) => literal.to_tokens(generator),
            GenericArgsConst::SimplePathSegment(segment) => segment.to_tokens(generator),
            GenericArgsConst::DashLiteral(dash, literal) => {
                dash.to_tokens(generator);
                literal.to_tokens(generator);
            }
        }
    }
}
