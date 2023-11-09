use crate::tokens::{
    Ampersand, AmpersandEquals, Asterick, AsterickEquals, Caret, CaretEquals, Dash, DashEquals,
    DoubleAmpersand, DoubleEquals, DoubleLeftTriange, DoubleLeftTriangeEquals, DoubleRightTriangle,
    DoubleRightTriangleEquals, DoubleVerticalBar, Exclamation, LeftTriangle, LeftTriangleEquals,
    Percent, PercentEquals, Plus, PlusEquals, RightTriangle, RightTriangleEquals, Slash,
    SlashEquals, VerticalBar, VerticalBarEquals,
};

#[derive(Clone)]
pub enum BinaryOperator {
    Add(Plus),
    Subtract(Dash),
    Multiply(Asterick),
    Divide(Slash),
    Remainder(Percent),
    And(DoubleAmpersand),
    Or(DoubleVerticalBar),
    BitXOr(Caret),
    BitAnd(Ampersand),
    BitOr(VerticalBar),
    ShiftLeft(DoubleLeftTriange),
    ShiftRight(DoubleRightTriangle),
    Equal(DoubleEquals),
    LessThan(LeftTriangle),
    LessThanOrEqualTo(LeftTriangleEquals),
    NotEqual(Exclamation),
    GreaterThanOrEqualTo(RightTriangleEquals),
    GreaterThan(RightTriangle),
    AddAssign(PlusEquals),
    SubtractAssign(DashEquals),
    MultiplyAssign(AsterickEquals),
    DivideAssign(SlashEquals),
    RemainderAssign(PercentEquals),
    BitXorAssign(CaretEquals),
    BitAndAssign(AmpersandEquals),
    BitOrAssign(VerticalBarEquals),
    ShiftLeftAssign(DoubleLeftTriangeEquals),
    ShiftRightAssign(DoubleRightTriangleEquals),
}
