use crate::{
    parsing::{Parse, Parser},
    tokens::{Ident, Literal, TokenStream},
    Result,
};

mod array;
mod assign;
mod r#async;
mod r#await;
mod binary;
mod block;
mod r#break;
mod call;
mod cast;
mod closure;
mod r#const;
mod r#continue;
mod field;
mod field_value;
mod for_loop;
mod group;
mod r#if;
mod index;
mod infer;
mod r#let;
mod literal;
mod r#loop;
mod r#macro;
mod r#match;
mod method_call;
mod parentheses;
mod path;
mod range;
mod reference;
mod repeat;
mod r#return;
mod r#struct;
mod r#try;
mod try_block;
mod tuple;
mod unary;
mod r#unsafe;
mod r#while;
mod r#yield;

pub use array::ExpressionArray;
pub use assign::ExpressionAssign;
pub use binary::{BinaryOperator, ExpressionBinary};
pub use block::ExpressionBlock;
pub use call::ExpressionCall;
pub use cast::ExpressionCast;
pub use closure::ExpressionClosure;
pub use field::ExpressionField;
pub use field_value::FieldValue;
pub use for_loop::ExpressionForLoop;
pub use group::ExpressionGroup;
pub use index::ExpressionIndex;
pub use infer::ExpressionInfer;
pub use literal::ExpressionLiteral;
pub use method_call::ExpressionMethodCall;
pub use parentheses::ExpressionParentheses;
pub use path::ExpressionPath;
pub use r#async::ExpressionAsync;
pub use r#await::ExpressionAwait;
pub use r#break::ExpressionBreak;
pub use r#const::ExpressionConstant;
pub use r#continue::ExpressionContinue;
pub use r#if::ExpressionIf;
pub use r#let::ExpressionLet;
pub use r#loop::ExpressionLoop;
pub use r#macro::ExpressionMacro;
pub use r#match::{Arm, ExpressionMatch};
pub use r#return::ExpressionReturn;
pub use r#struct::ExpressionStruct;
pub use r#try::ExpressionTry;
pub use r#unsafe::ExpressionUnsafe;
pub use r#while::ExpressionWhile;
pub use r#yield::ExpressionYield;
pub use range::{ExpressionRange, RangeLimits};
pub use reference::ExpressionReference;
pub use repeat::ExpressionRepeat;
pub use try_block::ExpressionTryBlock;
pub use tuple::ExpressionTuple;
pub use unary::{ExpressionUnary, UnaryOperator};

#[non_exhaustive]
#[derive(Clone)]
pub enum Expression {
    Array(ExpressionArray),
    Assign(ExpressionAssign),
    Async(ExpressionAsync),
    Await(ExpressionAwait),
    Binary(ExpressionBinary),
    Block(ExpressionBlock),
    Break(ExpressionBreak),
    Call(ExpressionCall),
    Cast(ExpressionCast),
    Closure(ExpressionClosure),
    Constant(ExpressionConstant),
    Continue(ExpressionContinue),
    Field(ExpressionField),
    ForLoop(ExpressionForLoop),
    Group(ExpressionGroup),
    If(ExpressionIf),
    Index(ExpressionIndex),
    Infer(ExpressionInfer),
    Let(ExpressionLet),
    Literal(ExpressionLiteral),
    Loop(ExpressionLoop),
    Macro(ExpressionMacro),
    Match(ExpressionMatch),
    MethodCall(ExpressionMethodCall),
    Parentheses(ExpressionParentheses),
    Path(ExpressionPath),
    Range(ExpressionRange),
    Reference(ExpressionReference),
    Repeat(ExpressionRepeat),
    Return(ExpressionReturn),
    Struct(ExpressionStruct),
    Try(ExpressionTry),
    TryBlock(ExpressionTryBlock),
    Tuple(ExpressionTuple),
    Unary(ExpressionUnary),
    Unsafe(ExpressionUnsafe),
    Verbatim(TokenStream),
    While(ExpressionWhile),
    Yield(ExpressionYield),
}

impl Expression {
    pub(crate) fn constant(parser: &mut Parser) -> Result<Expression> {
        if parser.peek::<Literal>() {
            return Ok(Expression::Literal(parser.parse()?));
        }

        if parser.peek::<Ident>() {
            todo!()
        }

        todo!()
    }
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        todo!()
    }
}
