use std::{fmt::Display, ops::Range};

use logos::Logos;

/// A token lexed by the Flux lexer.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex("[A-Za-z_][A-Za-z_0-9]*", |lex| lex.slice().to_string())]
    Ident(String),

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

    #[token("&")]
    TokenAnd,

    #[token("|")]
    TokenOr,

    #[token("!")]
    TokenNot,

    #[token("&&")]
    TokenLogicalAnd,

    #[token("||")]
    TokenLogicalOr,

    #[token("+=")]
    TokenPlusEq,

    #[token("-=")]
    TokenMinusEq,

    #[token("!=")]
    TokenNe,

    #[token("++")]
    TokenIncrement,

    #[token("--")]
    TokenDecrement,

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

    #[token(":")]
    TokenColon,

    #[token("->")]
    TokenArrow,

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

    #[token("extern")]
    KeywordExtern,

    #[token("match")]
    KeywordMatch,

    // literals - these only consume strings as the actual parsing should be handled by the parser
    // crate.
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    LiteralInt(i64),

    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse())]
    LiteralFloat(f64),

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().to_string())]
    LiteralStr(String),

    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\')'"#, |lex| lex.slice().parse())]
    LiteralChar(char),

    #[regex("(true)|(false)", |lex| lex.slice().parse(), priority = 2)]
    LiteralBool(bool),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Error => "unexpected token",
                Token::Ident(_) => "identifier",
                Token::TokenAssign => "=",
                Token::TokenEq => "==",
                Token::TokenPlus => "+",
                Token::TokenMinus => "-",
                Token::TokenSlash => "/",
                Token::TokenStar => "*",
                Token::TokenPercent => "%",
                Token::TokenPlusEq => "+=",
                Token::TokenMinusEq => "-=",
                Token::TokenIncrement => "++",
                Token::TokenDecrement => "--",
                Token::TokenNe => "!=",
                Token::TokenBraceLeft => "{",
                Token::TokenBraceRight => "}",
                Token::TokenBracketLeft => "[",
                Token::TokenBracketRight => "]",
                Token::TokenParenthesisLeft => "(",
                Token::TokenParenthesisRight => ")",
                Token::TokenComma => ",",
                Token::TokenColon => ":",
                Token::TokenArrow => "->",
                Token::KeywordLet => "let",
                Token::KeywordMut => "mut",
                Token::KeywordConst => "const",
                Token::KeywordIf => "if",
                Token::KeywordElse => "else",
                Token::KeywordReturn => "return",
                Token::KeywordLoop => "loop",
                Token::KeywordDo => "do",
                Token::KeywordWhile => "while",
                Token::KeywordFor => "for",
                Token::KeywordBreak => "break",
                Token::KeywordImport => "import",
                Token::KeywordFrom => "from",
                Token::KeywordAs => "as",
                Token::KeywordExport => "export",
                Token::KeywordExtern => "extern",
                Token::KeywordMatch => "match",
                Token::LiteralInt(_) => "integer",
                Token::LiteralFloat(_) => "float",
                Token::LiteralStr(_) => "str",
                Token::LiteralChar(_) => "char",
                Token::LiteralBool(_) => "bool",
                Token::TokenAnd => "&",
                Token::TokenOr => "|",
                Token::TokenNot => "!",
                Token::TokenLogicalAnd => "&&",
                Token::TokenLogicalOr => "||",
            }
        )
    }
}

/// Type representing the token stream for parsed source.
pub type TokenStream = Vec<SpannedToken>;

pub type SpannedToken = (Token, Range<usize>);

/// Lex the target source.
pub fn lex<S: AsRef<str>>(s: S) -> Result<TokenStream, TokenStream> {
    let lex = Token::lexer(s.as_ref());
    // map tokens
    let tokens = lex.spanned().collect::<Vec<_>>();
    // check for errors
    for pair in &tokens {
        if matches!(pair.0, Token::Error) {
            return Err(tokens);
        }
    }
    Ok(tokens)
}
