use crate::ast::{Expression, ExpressionKind, OuterAttribute};

impl<'a, T: Into<ExpressionKind<'a>>> From<T> for Expression<'a> {
    fn from(kind: T) -> Self {
        Expression::new(Vec::new(), kind)
    }
}

impl<'a, T: Into<ExpressionKind<'a>>> From<(OuterAttribute<'a>, T)> for Expression<'a> {
    fn from(value: (OuterAttribute<'a>, T)) -> Self {
        Expression::new(vec![value.0], value.1)
    }
}

impl<'a, V: Into<Vec<OuterAttribute<'a>>>, T: Into<ExpressionKind<'a>>> From<(V, T)>
    for Expression<'a>
{
    fn from(value: (V, T)) -> Self {
        Expression::new(value.0.into(), value.1)
    }
}
