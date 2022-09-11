use fluxc_lexer::{lex, Token};
use pretty_assertions::assert_eq;

#[test]
fn test_lex_fibonacci() {
    let src = include_str!("./fibonacci.flx");
    let tokens = lex(src);
    assert_eq!(
        Ok(vec![
            (Token::KeywordLet, 0..3),
            (Token::Ident, 4..5),
            (Token::TokenComma, 5..6),
            (Token::Ident, 7..8),
            (Token::TokenAssign, 9..10),
            (Token::LiteralInt, 11..12),
            (Token::KeywordLet, 13..16),
            (Token::Ident, 17..18),
            (Token::TokenAssign, 19..20),
            (Token::LiteralInt, 21..22),
            (Token::KeywordLoop, 23..27),
            (Token::TokenBraceLeft, 28..29),
            (Token::Ident, 34..35),
            (Token::TokenAssign, 36..37),
            (Token::Ident, 38..39),
            (Token::Ident, 44..45),
            (Token::TokenPlusEq, 46..48),
            (Token::Ident, 49..50),
            (Token::Ident, 55..56),
            (Token::TokenAssign, 57..58),
            (Token::Ident, 59..60),
            (Token::Ident, 65..70),
            (Token::Ident, 71..72),
            (Token::TokenBraceRight, 73..74)
        ]),
        tokens
    )
}
