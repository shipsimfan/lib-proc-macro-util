use crate::{tokens::Punctuation, Generator, Parse, Parser, Result, Spacing, Span, ToTokens};

mod r#macro;

r#macro::punctuation![
    "..." DotDotDot
    "..=" DotDotEq
    "<<=" ShlEq
    ">>=" ShrEq

    "&&" AndAnd
    "&=" AndEq
    "^=" CaretEq
    ".." DotDot
    "==" EqEq
    "=>" FatArrow
    ">=" Ge
    "<-" LArrow
    "<=" Le
    "-=" MinusEq
    "!=" Ne
    "|=" OrEq
    "||" OrOr
    "::" PathSep
    "%=" PercentEq
    "+=" PlusEq
    "->" RArrow
    "<<" Shl
    ">>" Shr
    "/=" SlashEq
    "*=" StarEq

    "&" And
    "@" At
    "^" Caret
    ":" Colon
    "," Comma
    "$" Dollar
    "." Dot
    "=" Eq
    ">" Gt
    "<" Lt
    "-" Minus
    "!" Not
    "|" Or
    "%" Percent
    "+" Plus
    "#" Pound
    "?" Question
    ";" Semi
    "/" Slash
    "*" Star
    "~" Tilde
    "_" Underscore
];
