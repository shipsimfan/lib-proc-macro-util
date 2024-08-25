/// Creates a token with span `Span::call_site()`
#[macro_export]
macro_rules! Token {
    [const] => { $crate::tokens::Const };
    [crate] => { $crate::tokens::Crate };
    [dyn] => {$crate::tokens::Dyn };
    [fn] => { $crate::tokens::Fn };
    [let] => { $crate::tokens::Let };
    [mut] => { $crate::tokens::Mut };
    [pub] => { $crate::tokens::Pub };
    [self] => { $crate::tokens::LowerSelf };
    [Self] => { $crate::tokens::UpperSelf };
    [struct] => { $crate::tokens::Struct };
    [super] => { $crate::tokens::Super };
    [&] => { $crate::tokens::Ampersand };
    ['_] => { $crate::tokens::Apostrophe };
    [*] => { $crate::tokens::Asterick };
    [:] => { $crate::tokens::Colon };
    [::] => { $crate::tokens::DoubleColon };
    [,] => { $crate::tokens::Comma };
    [.] => { $crate::tokens::Dot };
    [=] => { $crate::tokens::Equals };
    [!] => { $crate::tokens::Exclamation };
    [=>] => { $crate::tokens::FatRightArrow };
    [#] => { $crate::tokens::Hash };
    [<] => { $crate::tokens::LeftTriangle };
    [?] => { $crate::tokens::QuestionMark };
    [->] => { $crate::tokens::RightArrow };
    [>] => { $crate::tokens::RightTriangle };
    [;] => { $crate::tokens::SemiColon };
    [_] => { $crate::tokens::Underscore };
    [|] => { $crate::tokens::VerticalBar };
    [/] => { $crate::tokens::ForwardSlash };
    [+] => { $crate::tokens::Plus };
}
