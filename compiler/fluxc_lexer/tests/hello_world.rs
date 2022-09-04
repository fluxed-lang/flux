use fluxc_span::Span;
use pretty_assertions::assert_eq;

use fluxc_lexer::{lex, Token};

#[test]
fn test_lex_hello_world() {
	let src = include_str!("./hello-world.flx");
	let tokens = lex(src);
	assert_eq!(Ok(vec![
		(Token::Ident, Span::from_str(src).restrict_range(0, 5)),
		(Token::Str, Span::from_str(src).restrict_range(6, 21))
	]), tokens)
}
