use crate::ast::GenericArgsConst;

impl<'a> GenericArgsConst<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericArgsConst<'static> {
        match self {
            GenericArgsConst::Block(block) => GenericArgsConst::Block(block.into_static()),
            GenericArgsConst::Literal(literal) => GenericArgsConst::Literal(literal.into_static()),
            GenericArgsConst::SimplePathSegment(segment) => {
                GenericArgsConst::SimplePathSegment(segment.into_static())
            }
            GenericArgsConst::DashLiteral(dash, literal) => {
                GenericArgsConst::DashLiteral(dash, literal.into_static())
            }
        }
    }
}
