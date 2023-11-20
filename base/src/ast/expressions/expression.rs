use crate::{
    ast::{FunctionCallExpression, MacroCallExpression},
    Parse, ToTokens,
};

/// A Rust expression
#[derive(Clone)]
pub enum Expression<'a> {
    /// An invocation of a function
    FunctionCall(FunctionCallExpression<'a>),

    /// An invocation of a macro
    MacroCall(MacroCallExpression<'a>),
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        if parser.peek::<MacroCallExpression>() {
            parser
                .parse()
                .map(|macro_call| Expression::MacroCall(macro_call))
        } else if parser.peek::<FunctionCallExpression>() {
            parser
                .parse()
                .map(|function_call| Expression::FunctionCall(function_call))
        } else {
            Err(parser.error("expected an expression"))
        }
    }
}

impl<'a> ToTokens for Expression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        match self {
            Expression::FunctionCall(function_call) => function_call.to_tokens(generator),
            Expression::MacroCall(macro_call) => macro_call.to_tokens(generator),
        }
    }
}
