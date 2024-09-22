use crate::ast::{ExpressionWithoutBlock, ExpressionWithoutBlockKind, OuterAttribute};

impl<'a, T: Into<ExpressionWithoutBlockKind<'a>>> From<T> for ExpressionWithoutBlock<'a> {
    fn from(kind: T) -> Self {
        ExpressionWithoutBlock::new(Vec::new(), kind)
    }
}

impl<'a, T: Into<ExpressionWithoutBlockKind<'a>>> From<(OuterAttribute<'a>, T)>
    for ExpressionWithoutBlock<'a>
{
    fn from(value: (OuterAttribute<'a>, T)) -> Self {
        ExpressionWithoutBlock::new(vec![value.0], value.1)
    }
}

impl<'a, V: Into<Vec<OuterAttribute<'a>>>, T: Into<ExpressionWithoutBlockKind<'a>>> From<(V, T)>
    for ExpressionWithoutBlock<'a>
{
    fn from(value: (V, T)) -> Self {
        ExpressionWithoutBlock::new(value.0.into(), value.1)
    }
}
