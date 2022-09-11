use fluxc_lexer::{lex, Token};
use pretty_assertions::assert_eq;

#[test]
fn test_lex_fibonacci() {
    let src = include_str!("./fibonacci.flx");
    let tokens = lex(src);
    assert_eq!(
        Ok(vec![
            (Token::KeywordLet, 0..3),
            (Token::Ident("x".to_string()), 4..5),
            (Token::TokenComma, 5..6),
            (Token::Ident("y".to_string()), 7..8),
            (Token::TokenAssign, 9..10),
            (Token::LiteralInt("1".to_string()), 11..12),
            (Token::KeywordLet, 13..16),
            (Token::Ident("z".to_string()), 17..18),
            (Token::TokenAssign, 19..20),
            (Token::LiteralInt("0".to_string()), 21..22),
            (Token::KeywordLoop, 23..27),
            (Token::TokenBraceLeft, 28..29),
            (Token::Ident("z".to_string()), 34..35),
            (Token::TokenAssign, 36..37),
            (Token::Ident("x".to_string()), 38..39),
            (Token::Ident("x".to_string()), 44..45),
            (Token::TokenPlusEq, 46..48),
            (Token::Ident("y".to_string()), 49..50),
            (Token::Ident("y".to_string()), 55..56),
            (Token::TokenAssign, 57..58),
            (Token::Ident("z".to_string()), 59..60),
            (Token::Ident("print".to_string()), 65..70),
            (Token::Ident("z".to_string()), 71..72),
            (Token::TokenBraceRight, 73..74)
        ]),
        tokens
    )
}
