use crate::ast::ExpressionWithoutBlock;

impl<'a> std::fmt::Display for ExpressionWithoutBlock<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionWithoutBlock::Literal(literal) => literal.fmt(f),
        }
    }
}
