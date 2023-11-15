/// Creates a token with span `Span::call_site()`
#[macro_export]
macro_rules! Token {
    [const] => { $crate::tokens::Const::default() };
    [let] => { $crate::tokens::Let::default() };
    [mut] => { $crate::tokens::Mut::default() };
    [&] => { $crate::tokens::Ampersand::default() };
    [:] => { $crate::tokens::Colon::default() };
    [::] => { $crate::tokens::DoubleColon::default() };
    [,] => { $crate::tokens::Comma::default() };
    [.] => { $crate::tokens::Dot::default() };
    [=] => { $crate::tokens::Equals::default() };
    [!] => { $crate::tokens::Exclamation::default() };
    [;] => { $crate::tokens::SemiColon::default() };
}
