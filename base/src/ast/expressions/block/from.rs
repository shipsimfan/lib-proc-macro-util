use crate::ast::{expressions::BlockExpression, ExpressionWithoutBlock, InnerAttribute, Statement};

impl<'a>
    From<(
        Vec<InnerAttribute<'a>>,
        Vec<Statement>,
        Option<ExpressionWithoutBlock<'a>>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            Vec<InnerAttribute<'a>>,
            Vec<Statement>,
            Option<ExpressionWithoutBlock<'a>>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: value.1,
            end: value.2.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<(Vec<InnerAttribute<'a>>, Vec<Statement>)> for BlockExpression<'a> {
    fn from(value: (Vec<InnerAttribute<'a>>, Vec<Statement>)) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: value.1,
            end: None,
        }
    }
}

impl<'a>
    From<(
        Vec<InnerAttribute<'a>>,
        Vec<Statement>,
        ExpressionWithoutBlock<'a>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            Vec<InnerAttribute<'a>>,
            Vec<Statement>,
            ExpressionWithoutBlock<'a>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: value.1,
            end: Some(Box::new(value.2)),
        }
    }
}

impl<'a>
    From<(
        InnerAttribute<'a>,
        Vec<Statement>,
        Option<ExpressionWithoutBlock<'a>>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            InnerAttribute<'a>,
            Vec<Statement>,
            Option<ExpressionWithoutBlock<'a>>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: value.1,
            end: value.2.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<(InnerAttribute<'a>, Vec<Statement>)> for BlockExpression<'a> {
    fn from(value: (InnerAttribute<'a>, Vec<Statement>)) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: value.1,
            end: None,
        }
    }
}

impl<'a>
    From<(
        InnerAttribute<'a>,
        Vec<Statement>,
        ExpressionWithoutBlock<'a>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            InnerAttribute<'a>,
            Vec<Statement>,
            ExpressionWithoutBlock<'a>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: value.1,
            end: Some(Box::new(value.2)),
        }
    }
}

impl<'a> From<(Vec<Statement>, Option<ExpressionWithoutBlock<'a>>)> for BlockExpression<'a> {
    fn from(value: (Vec<Statement>, Option<ExpressionWithoutBlock<'a>>)) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: value.0,
            end: value.1.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<Vec<Statement>> for BlockExpression<'a> {
    fn from(statements: Vec<Statement>) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements,
            end: None,
        }
    }
}

impl<'a> From<(Vec<Statement>, ExpressionWithoutBlock<'a>)> for BlockExpression<'a> {
    fn from(value: (Vec<Statement>, ExpressionWithoutBlock<'a>)) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: value.0,
            end: Some(Box::new(value.1)),
        }
    }
}

impl<'a>
    From<(
        Vec<InnerAttribute<'a>>,
        Statement,
        Option<ExpressionWithoutBlock<'a>>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            Vec<InnerAttribute<'a>>,
            Statement,
            Option<ExpressionWithoutBlock<'a>>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: vec![value.1],
            end: value.2.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<(Vec<InnerAttribute<'a>>, Statement)> for BlockExpression<'a> {
    fn from(value: (Vec<InnerAttribute<'a>>, Statement)) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: vec![value.1],
            end: None,
        }
    }
}

impl<'a>
    From<(
        Vec<InnerAttribute<'a>>,
        Statement,
        ExpressionWithoutBlock<'a>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            Vec<InnerAttribute<'a>>,
            Statement,
            ExpressionWithoutBlock<'a>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: vec![value.1],
            end: Some(Box::new(value.2)),
        }
    }
}

impl<'a>
    From<(
        InnerAttribute<'a>,
        Statement,
        Option<ExpressionWithoutBlock<'a>>,
    )> for BlockExpression<'a>
{
    fn from(
        value: (
            InnerAttribute<'a>,
            Statement,
            Option<ExpressionWithoutBlock<'a>>,
        ),
    ) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: vec![value.1],
            end: value.2.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<(InnerAttribute<'a>, Statement)> for BlockExpression<'a> {
    fn from(value: (InnerAttribute<'a>, Statement)) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: vec![value.1],
            end: None,
        }
    }
}

impl<'a> From<(InnerAttribute<'a>, Statement, ExpressionWithoutBlock<'a>)> for BlockExpression<'a> {
    fn from(value: (InnerAttribute<'a>, Statement, ExpressionWithoutBlock<'a>)) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: vec![value.1],
            end: Some(Box::new(value.2)),
        }
    }
}

impl<'a> From<(Statement, Option<ExpressionWithoutBlock<'a>>)> for BlockExpression<'a> {
    fn from(value: (Statement, Option<ExpressionWithoutBlock<'a>>)) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: vec![value.0],
            end: value.1.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<Statement> for BlockExpression<'a> {
    fn from(statement: Statement) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: vec![statement],
            end: None,
        }
    }
}

impl<'a> From<(Statement, ExpressionWithoutBlock<'a>)> for BlockExpression<'a> {
    fn from(value: (Statement, ExpressionWithoutBlock<'a>)) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: vec![value.0],
            end: Some(Box::new(value.1)),
        }
    }
}

impl<'a> From<(Vec<InnerAttribute<'a>>, Option<ExpressionWithoutBlock<'a>>)>
    for BlockExpression<'a>
{
    fn from(value: (Vec<InnerAttribute<'a>>, Option<ExpressionWithoutBlock<'a>>)) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: Vec::new(),
            end: value.1.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<Vec<InnerAttribute<'a>>> for BlockExpression<'a> {
    fn from(attributes: Vec<InnerAttribute<'a>>) -> Self {
        BlockExpression {
            attributes,
            statements: Vec::new(),
            end: None,
        }
    }
}

impl<'a> From<(Vec<InnerAttribute<'a>>, ExpressionWithoutBlock<'a>)> for BlockExpression<'a> {
    fn from(value: (Vec<InnerAttribute<'a>>, ExpressionWithoutBlock<'a>)) -> Self {
        BlockExpression {
            attributes: value.0,
            statements: Vec::new(),
            end: Some(Box::new(value.1)),
        }
    }
}

impl<'a> From<(InnerAttribute<'a>, Option<ExpressionWithoutBlock<'a>>)> for BlockExpression<'a> {
    fn from(value: (InnerAttribute<'a>, Option<ExpressionWithoutBlock<'a>>)) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: Vec::new(),
            end: value.1.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<InnerAttribute<'a>> for BlockExpression<'a> {
    fn from(attribute: InnerAttribute<'a>) -> Self {
        BlockExpression {
            attributes: vec![attribute],
            statements: Vec::new(),
            end: None,
        }
    }
}

impl<'a> From<(InnerAttribute<'a>, ExpressionWithoutBlock<'a>)> for BlockExpression<'a> {
    fn from(value: (InnerAttribute<'a>, ExpressionWithoutBlock<'a>)) -> Self {
        BlockExpression {
            attributes: vec![value.0],
            statements: Vec::new(),
            end: Some(Box::new(value.1)),
        }
    }
}

impl<'a> From<Option<ExpressionWithoutBlock<'a>>> for BlockExpression<'a> {
    fn from(value: Option<ExpressionWithoutBlock<'a>>) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: Vec::new(),
            end: value.map(|expression| Box::new(expression)),
        }
    }
}

impl<'a> From<()> for BlockExpression<'a> {
    fn from(_: ()) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: Vec::new(),
            end: None,
        }
    }
}

impl<'a> From<ExpressionWithoutBlock<'a>> for BlockExpression<'a> {
    fn from(value: ExpressionWithoutBlock<'a>) -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: Vec::new(),
            end: Some(Box::new(value)),
        }
    }
}
