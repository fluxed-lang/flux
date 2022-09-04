use fluxc_span::Span;
use logos::Logos;

/// A token lexed by the Flux lexer.
#[derive(Logos, Debug, PartialEq, Eq, Clone, Hash)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

	#[regex("[A-Za-z_][A-Za-z_0-9]*")]
	Ident,

	// tokens
	#[token("=")]
	TokenAssign,

	#[token("==")]
	TokenEq,

	#[token("+")]
	TokenPlus,

	#[token("-")]
	TokenMinus,

	#[token("/")]
	TokenSlash,

	#[token("*")]
	TokenStar,

	#[token("%")]
	TokenPercent,

	#[token("+=")]
	TokenPlusEq,

	#[token("-=")]
	TokenMinusEq,

	#[token("{")]
	TokenBraceLeft,

	#[token("}")]
	TokenBraceRight,

	#[token("[")]
	TokenBracketLeft,

	#[token("]")]
	TokenBracketRight,

	#[token("(")]
	TokenParenthesisLeft,

	#[token(")")]
	TokenParenthesisRight,

	#[token(",")]
	TokenComma,

	// keywords
	#[token("let")]
	KeywordLet,

	#[token("mut")]
	KeywordMut,

	#[token("const")]
	KeywordConst,

	#[token("if")]
	KeywordIf,

	#[token("else")]
	KeywordElse,

	#[token("return")]
	KeywordReturn,

	#[token("loop")]
	KeywordLoop,

	#[token("do")]
	KeywordDo,

	#[token("while")]
	KeywordWhile,

	#[token("for")]
	KeywordFor,

	#[token("break")]
	KeywordBreak,

	#[token("import")]
	KeywordImport,

	#[token("from")]
	KeywordFrom,

	#[token("as")]
	KeywordAs,

	#[token("export")]
	KeywordExport,

	// literals
    #[regex("-?[0-9]+")]
    Integer,

    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+")]
    Float,

	#[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    Str,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")""#)]
	Char
}

/// Type representing a token paired with its associated span.
pub type TokenPair = (Token, Span);

/// Type representing the token stream for parsed source.
pub type TokenStream = Vec<TokenPair>;

/// Lex the target source.
pub fn lex<S: AsRef<str>>(s: S) -> Result<TokenStream, TokenStream> {
    let lex = Token::lexer(s.as_ref());
    let src_span = Span::from_str(s.as_ref());
    // map tokens
    let tokens = lex
        .spanned()
        .map(|(token, span)| (token, src_span.restrict_range(span.start, span.end)))
        .collect::<Vec<_>>();
    // check for errors
    for (token, _) in &tokens {
        if matches!(token, Token::Error) {
            return Err(tokens);
        }
    }
    Ok(tokens)
}
