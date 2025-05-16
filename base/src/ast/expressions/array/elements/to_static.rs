use crate::ast::expressions::ArrayElements;

impl<'a> ArrayElements<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ArrayElements<'static> {
        match self {
            ArrayElements::Every {
                first,
                remaining,
                last,
            } => ArrayElements::Every {
                first: Box::new(first.into_static()),
                remaining: remaining
                    .into_iter()
                    .map(|(comma, expression)| (comma, expression.into_static()))
                    .collect(),
                last,
            },
            ArrayElements::Uniform {
                element,
                semi,
                count,
            } => ArrayElements::Uniform {
                element: Box::new(element.into_static()),
                semi,
                count: Box::new(count.into_static()),
            },
        }
    }
}
