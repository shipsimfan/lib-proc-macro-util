use crate::ast::{ExpressionWithBlock, ExpressionWithBlockKind, OuterAttribute};

impl<'a, T: Into<ExpressionWithBlockKind<'a>>> From<T> for ExpressionWithBlock<'a> {
    fn from(kind: T) -> Self {
        ExpressionWithBlock::new(Vec::new(), kind)
    }
}

impl<'a, T: Into<ExpressionWithBlockKind<'a>>> From<(OuterAttribute<'a>, T)>
    for ExpressionWithBlock<'a>
{
    fn from(value: (OuterAttribute<'a>, T)) -> Self {
        ExpressionWithBlock::new(vec![value.0], value.1)
    }
}

impl<'a, V: Into<Vec<OuterAttribute<'a>>>, T: Into<ExpressionWithBlockKind<'a>>> From<(V, T)>
    for ExpressionWithBlock<'a>
{
    fn from(value: (V, T)) -> Self {
        ExpressionWithBlock::new(value.0.into(), value.1)
    }
}
