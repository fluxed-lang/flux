use fluxc_span::Span;
use pretty_assertions::assert_eq;

use fluxc_lexer::{lex, Token};

#[test]
fn test_lex_fibonacci() {
	let src = include_str!("./fibonacci.flx");
	let span = Span::from_str(src);
	let tokens = lex(src);
	assert_eq!(Ok(vec![
		(Token::KeywordLet, span.restrict_range(0, 3)),
		(Token::Ident, span.restrict_range(4, 5)),
		(Token::TokenComma, span.restrict_range(5, 6)),
		(Token::Ident, span.restrict_range(7, 8)),
		(Token::TokenAssign, span.restrict_range(9, 10)),
		(Token::Integer, span.restrict_range(11, 12)),
		(Token::KeywordLet, span.restrict_range(13, 16)),
		(Token::Ident, span.restrict_range(17, 18)),
		(Token::TokenAssign, span.restrict_range(19, 20)),
		(Token::Integer, span.restrict_range(21, 22)),
		(Token::KeywordLoop, span.restrict_range(23, 27)),
		(Token::TokenBraceLeft, span.restrict_range(28, 29)),
		(Token::Ident, span.restrict_range(34, 35)),
		(Token::TokenAssign, span.restrict_range(36, 37)),
		(Token::Ident, span.restrict_range(38, 39)),
		(Token::Ident, span.restrict_range(44, 45)),
		(Token::TokenPlusEq, span.restrict_range(46, 48)),
		(Token::Ident, span.restrict_range(49, 50)),
		(Token::Ident, span.restrict_range(55, 56)),
		(Token::TokenAssign, span.restrict_range(57, 58)),
		(Token::Ident, span.restrict_range(59, 60)),
		(Token::Ident, span.restrict_range(65, 70)),
		(Token::Ident, span.restrict_range(71, 72)),
		(Token::TokenBraceRight, span.restrict_range(73, 74))
	]), tokens)
}
