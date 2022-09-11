use fluxc_lexer::{lex, Token};
use pretty_assertions::assert_eq;

#[test]
fn test_lex_hello_world() {
    let src = include_str!("./hello-world.flx");
    let tokens = lex(src);
    assert_eq!(Ok(vec![(Token::Ident, 0..5), (Token::LiteralStr, 6..21)]), tokens)
}
