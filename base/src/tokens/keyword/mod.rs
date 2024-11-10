use crate::{
    supported_languages::*,
    tokens::{Identifier, TokenTree},
    Generator, Parse, Parser, Result, Span, ToTokens,
};
use i18n_translation::m;

mod r#macro;

r#macro::keywords! [
    "abstract" Abstract
    "as" As
    "async" Async
    "await" Await
    "become" Become
    "box" Box
    "break" Break
    "const" Const
    "continue" Continue
    "crate" Crate
    "do" Do
    "dyn" Dyn
    "else" Else
    "enum" Enum
    "extern" Extern
    "false" False
    "final" Final
    "fn" Fn
    "for" For
    "if" If
    "impl" Impl
    "in" In
    "let" Let
    "loop" Loop
    "macro" Macro
    "match" Match
    "mod" Mod
    "move" Move
    "mut" Mut
    "override" Override
    "priv" Priv
    "pub" Pub
    "ref" Ref
    "return" Return
    "self" LowerSelf
    "Self" UpperSelf
    "static" Static
    "struct" Struct
    "super" Super
    "trait" Trait
    "true" True
    "try" Try
    "type" Type
    "typeof" TypeOf
    "unsafe" Unsafe
    "unsized" Unsized
    "use" Use
    "virtual" Virtual
    "where" Where
    "while" While
    "yield" Yield
];
