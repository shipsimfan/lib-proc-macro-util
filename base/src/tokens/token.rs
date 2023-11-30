/// Creates a token with span `Span::call_site()`
#[macro_export]
macro_rules! Token {
    [const] => { $crate::tokens::Const };
    [let] => { $crate::tokens::Let };
    [mut] => { $crate::tokens::Mut };
    [&] => { $crate::tokens::Ampersand };
    ['_] => { $crate::tokens::Apostrophe };
    [:] => { $crate::tokens::Colon };
    [::] => { $crate::tokens::DoubleColon };
    [,] => { $crate::tokens::Comma };
    [.] => { $crate::tokens::Dot };
    [=] => { $crate::tokens::Equals };
    [!] => { $crate::tokens::Exclamation };
    [#] => { $crate::tokens::Hash };
    [<] => { $crate::tokens::LeftTriangle };
    [?] => { $crate::tokens::QuestionMark };
    [->] => { $crate::tokens::RightArrow };
    [>] => { $crate::tokens::RightTriangle };
    [;] => { $crate::tokens::SemiColon };
    [_] => { $crate::tokens::Underscore };
    [|] => { $crate::tokens::VerticalBar };
}
