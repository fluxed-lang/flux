use fluxc_lexer::{lex, Token};
use pretty_assertions::assert_eq;

#[test]
fn test_lex_hello_world() {
    let src = include_str!("./hello-world.flx");
    let tokens = lex(src).unwrap();
    assert_eq!(
        vec![
            (Token::Ident("print".to_string()), 0..5),
            (Token::LiteralStr("\"hello, world!\"".to_string()), 6..21)
        ],
        tokens
    )
}
