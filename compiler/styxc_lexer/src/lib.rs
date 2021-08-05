use std::error::Error;

use logos::{Lexer, Logos};

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Private,
    Protected,
    Public,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Keyword {
    #[error]
    Error,

    /// The "let" token, used in declarations.
    #[token("let")]
    Let,

    /// The "const" token, used in constant declarations.
    #[token("const")]
    Const,

    /// The "for" keyword, used for declaring loops over iterators or ranges.
    #[token("for")]
    For,

    /// The "while" keyword, used for declaring a conditional loop.
    #[token("while")]
    While,

    /// The "loop" keyword, used for declaring a unconditional loop.
    #[token("loop")]
    Loop,

    /// The "break" keyword, used for breaking out of loops.
    #[token("break")]
    Break,

    /// The "continue" keyword, used for continuing to the next iteration of a loop.
    #[token("continue")]
    Continue,

    /// The "fn" keyword, used for declaring functions.
    #[token("fn")]
    Fn,

    /// The "async" keyword, used to declare an asynchronous function.
    #[token("async")]
    Async,

    /// The "return" keyword, used for returning from a function.
    #[token("return")]
    Return,


    /// The "await" keyword, used to wait for the current asynchronous operation to finish.
    #[token("await")]
    Await,

    /// The "import" keyword, used to import external modules.
    #[token("import")]
    Import,

    /// The "from" keyword, used when declaring an aliased or destructing import.
    #[token("from")]
    ImportFrom,

    /// The "type" keyword, used to declare a new type.
    #[token("type")]
    Type,

    /// The "if" keyword, used for declaring conditional statements.
    #[token("if")]
    If,

    /// The "else" keyword, used for declaring an "else" clause.
    #[token("else")]
    Else,

    /// The "match" keyword, used for declaring a pattern match.
    #[token("match")]
    Match,

    /// The "try" keyword, used for declaring a try/catch block.
    #[token("try")]
    Try,

    /// The "catch" keyword, used for declaring a catch block.
    #[token("catch")]
    Catch,

    /// The "finally" keyword, used for declaring a finally block.
    #[token("finally")]
    Finally,

    /// The "enum" keyword, used for declaring an enumeration.
    #[token("enum")]
    Enum,

    /// A visibility keyword, used for determining the visibility of a symbol.
    Visibility(Visibility),
}

impl Keyword {
    /// Parse the target slice into a keyword token.
    pub fn parse(slice: &str) -> Keyword {
        let mut lexer = Keyword::lexer(slice);
        // okay to unwrap - should panic if fails.
        lexer.next().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum Base {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}

impl Base {
    /// Parse the target slice into a string.
    fn parse(s: &str) -> Base {
        let mut slice = s.clone();
        // remove leading negation
        if slice.starts_with("-") {
            slice = &slice[1..];
        }
        // if length less than 2, cannot include base prefix
        if slice.len() < 2 {
            return Base::Decimal;
        }
        match &slice[0..2] {
            "0x" => Base::Hexadecimal,
            "0o" => Base::Octal,
            "0b" => Base::Binary,
            _ => Base::Decimal,
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum LiteralKind {
    #[error]
    Error,

    /// Represents any integer literal and its base.
    /// Matches both raw ints and integers with their base specified, e.g. 1234, or 0x1fff.
    #[regex("[+-]?[0-9]+", |lex| Base::parse(lex.slice()))]
    #[regex("[+-]?0x[0-9a-fA-F]+", |lex| Base::parse(lex.slice()) )]
    #[regex("[+-]?0d[0-9]+", |lex| Base::parse(lex.slice()) )]
    #[regex("[+-]?0o[0-7]+", |lex| Base::parse(lex.slice()) )]
    #[regex("[+-]?0b[01]+", |lex| Base::parse(lex.slice()) )]
    Int(Base),

    /// Represents any floating point literal. Matches both floating point and scientific notation.
    /// e.g. 0.1, 1e-10, 1.0e-10, 1.0e+10, 1.0e10, 1.0e-10
    #[regex("[+-]?[0-9]*\\.[0-9]+", |lex| Base::parse(lex.slice()))]
    #[regex("[+-]?[0-9]+e[+-]?[0-9]+", |lex| Base::parse(lex.slice()))]
    Float(Base),

    #[regex("'.'")]
    #[regex(r#"'\\u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]'"#)]
    Char,

    #[regex("\"([^\"]|(\\\\\"))*\"")]
    String,
}

impl LiteralKind {
    fn parse(s: &str) -> Result<LiteralKind, Box<dyn Error>> {
        let mut lex = LiteralKind::lexer(s);
        match lex.next() {
            Some(kind) => Ok(kind),
            None => Err("Failed to match literal kind".into()),
        }
    }
}

#[cfg(test)]
mod literal_kind {
    use super::*;

    #[test]
    fn test_int() {
        let mut lexer = LiteralKind::lexer("1234");
        assert_eq!(lexer.next(), Some(LiteralKind::Int(Base::Decimal)))
    }

    #[test]
    fn test_int_hex() {
        let mut lexer = LiteralKind::lexer("0x1234");
        assert_eq!(lexer.next(), Some(LiteralKind::Int(Base::Hexadecimal)))
    }

    #[test]
    fn test_int_oct() {
        let mut lexer = LiteralKind::lexer("0o1234");
        assert_eq!(lexer.next(), Some(LiteralKind::Int(Base::Octal)))
    }

    #[test]
    fn test_int_bin() {
        let mut lexer = LiteralKind::lexer("0b10100");
        assert_eq!(lexer.next(), Some(LiteralKind::Int(Base::Binary)))
    }

    #[test]
    fn test_float() {
        let mut lexer = LiteralKind::lexer("12.34");
        assert_eq!(lexer.next(), Some(LiteralKind::Float(Base::Decimal)))
    }

    #[test]
    fn test_float_exp() {
        let mut lexer = LiteralKind::lexer("12.34e-5");
        assert_eq!(lexer.next(), Some(LiteralKind::Float(Base::Decimal)));
        let mut lexer = LiteralKind::lexer("-432e+10");
        assert_eq!(lexer.next(), Some(LiteralKind::Float(Base::Decimal)));
    }

    #[test]
    fn test_char() {
        let mut lexer = LiteralKind::lexer("'a'");
        assert_eq!(lexer.next(), Some(LiteralKind::Char));
    }

    #[test]
    fn test_unicode_char() {
        let mut lexer = LiteralKind::lexer("'\\u1234'");
        assert_eq!(lexer.next(), Some(LiteralKind::Char));
    }

    #[test]
    fn test_string() {
        let mut lexer = LiteralKind::lexer("\"foo\"");
        assert_eq!(lexer.next(), Some(LiteralKind::String));
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[error]
    Error,

    /// Represents an identifier.
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Ident,

    /// Represents a keyword.
    #[token("let", |lex| Keyword::parse(lex.slice()))]
    #[token("const", |lex| Keyword::parse(lex.slice()))]
    #[token("for", |lex| Keyword::parse(lex.slice()))]
    #[token("while", |lex| Keyword::parse(lex.slice()))]
    #[token("loop", |lex| Keyword::parse(lex.slice()))]
    #[token("break", |lex| Keyword::parse(lex.slice()))]
    #[token("continue", |lex| Keyword::parse(lex.slice()))]
    #[token("fn", |lex| Keyword::parse(lex.slice()))]
    #[token("async", |lex| Keyword::parse(lex.slice()))]
    #[token("return", |lex| Keyword::parse(lex.slice()))]
    #[token("await", |lex| Keyword::parse(lex.slice()))]
    #[token("import", |lex| Keyword::parse(lex.slice()))]
    #[token("from", |lex| Keyword::parse(lex.slice()))]
    #[token("type", |lex| Keyword::parse(lex.slice()))]
    #[token("if", |lex| Keyword::parse(lex.slice()))]
    #[token("else", |lex| Keyword::parse(lex.slice()))]
    #[token("match", |lex| Keyword::parse(lex.slice()))]
    #[token("try", |lex| Keyword::parse(lex.slice()))]
    #[token("catch", |lex| Keyword::parse(lex.slice()))]
    #[token("finally", |lex| Keyword::parse(lex.slice()))]
    #[token("enum", |lex| Keyword::parse(lex.slice()))]
    Keyword(Keyword),

    /// Represents a generic whitespace character. This includes tabs, spaces, and newlines.
    #[regex("\\s+", logos::skip)]
    Whitespace,

    /// Represents a line comment.
    #[regex("#[^\n]+")]
    LineComment,

    /// Represents a block comment.
    #[regex(r#"/\*(.|\n)*\*/"#)]
    BlockComment,

    /// Matches a literal.
    #[regex("[+-]?[0-9]+", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("[+-]?0x[0-9a-fA-F]+", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("[+-]?0d[0-9]+", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("[+-]?0o[0-7]+", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("[+-]?0b[01]+", |lex|  LiteralKind::parse(lex.slice()))]
    #[regex("[+-]?[0-9]*\\.[0-9]+", |lex| LiteralKind::parse(lex.slice()) )]
    #[regex("[+-]?[0-9]+e[+-]?[0-9]+", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("'.'", |lex| LiteralKind::parse(lex.slice()))]
    #[regex("\"(\"|[^\"])*\"", |lex| LiteralKind::parse(lex.slice()))]
    Literal(LiteralKind),

    #[token(";")]
    Semi,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("=")]
    Eq,

    #[token("!")]
    Not,

    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("^")]
    Caret,

    #[token("~")]
    Tilde,

    #[token("?")]
    Question,

    #[token(":")]
    Colon,

    #[token(".")]
    Dot,

    #[token("@")]
    At,
}

#[cfg(test)]
mod token {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lexer = TokenKind::lexer("hello_world i_like_foxes_123 xbox360");
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_keyword() {
        let mut lexer = TokenKind::lexer("let const for while loop break continue fn async return await import from type if else match try catch finally enum");
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Let)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Const)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::For)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::While)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Loop)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Break)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Continue)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Fn)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Async)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Return)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Await)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Import)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::ImportFrom)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Type)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::If)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Else)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Match)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Try)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Catch)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Finally)));
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Enum)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_line_comment() {
        let mut lexer = TokenKind::lexer("# this is a comment\nhello_world");
        assert_eq!(lexer.next(), Some(TokenKind::LineComment));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_block_comment() {
        let mut lexer = TokenKind::lexer("/*this is a comment*/");
        assert_eq!(lexer.next(), Some(TokenKind::BlockComment));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_tokens() {
        let mut lexer = TokenKind::lexer(";{}[]()+-*/%=!&|<>^~?:.@");

        assert_eq!(lexer.next(), Some(TokenKind::Semi));
        assert_eq!(lexer.next(), Some(TokenKind::OpenBrace));
        assert_eq!(lexer.next(), Some(TokenKind::CloseBrace));
        assert_eq!(lexer.next(), Some(TokenKind::OpenBracket));
        assert_eq!(lexer.next(), Some(TokenKind::CloseBracket));
        assert_eq!(lexer.next(), Some(TokenKind::OpenParen));
        assert_eq!(lexer.next(), Some(TokenKind::CloseParen));
        assert_eq!(lexer.next(), Some(TokenKind::Plus));
        assert_eq!(lexer.next(), Some(TokenKind::Minus));
        assert_eq!(lexer.next(), Some(TokenKind::Star));
        assert_eq!(lexer.next(), Some(TokenKind::Slash));
        assert_eq!(lexer.next(), Some(TokenKind::Percent));
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        assert_eq!(lexer.next(), Some(TokenKind::Not));
        assert_eq!(lexer.next(), Some(TokenKind::And));
        assert_eq!(lexer.next(), Some(TokenKind::Or));
        assert_eq!(lexer.next(), Some(TokenKind::Lt));
        assert_eq!(lexer.next(), Some(TokenKind::Gt));
        assert_eq!(lexer.next(), Some(TokenKind::Caret));
        assert_eq!(lexer.next(), Some(TokenKind::Tilde));
        assert_eq!(lexer.next(), Some(TokenKind::Question));
        assert_eq!(lexer.next(), Some(TokenKind::Colon));
        assert_eq!(lexer.next(), Some(TokenKind::Dot));
        assert_eq!(lexer.next(), Some(TokenKind::At));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_expression() {
        let mut lexer = TokenKind::lexer("hello: int = 2;");
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Colon));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_binary_expression() {
        let mut lexer = TokenKind::lexer("1 + 2 * 3");
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Plus));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Star));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_assignment() {
        let mut lexer = TokenKind::lexer("let x = 2;");
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Let)));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_code() {
        let src = r#"
        fn main() {
            # assign variables
            let x = 1;
            let y = 2;
            let z = x + y;
        }"#;

        let mut lexer = TokenKind::lexer(src);

        // newline and indentation

        // fn
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Fn)));
        // space
        // main
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // ()
        assert_eq!(lexer.next(), Some(TokenKind::OpenParen));
        assert_eq!(lexer.next(), Some(TokenKind::CloseParen));
        // {
        assert_eq!(lexer.next(), Some(TokenKind::OpenBrace));

        // comment
        assert_eq!(lexer.next(), Some(TokenKind::LineComment));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Let)));
        // x
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        // 1
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Let)));
        // y
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        // 2
        assert_eq!(
            lexer.next(),
            Some(TokenKind::Literal(LiteralKind::Int(Base::Decimal)))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::Keyword(Keyword::Let)));
        // z
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        // x
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // +
        assert_eq!(lexer.next(), Some(TokenKind::Plus));
        // y
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Semi));

        // } and EOF
        assert_eq!(lexer.next(), Some(TokenKind::CloseBrace));
        assert_eq!(lexer.next(), None);
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub index: usize,
    pub len: usize,
    pub slice: String,
}

/// Represents a lexer error thrown at the target position.
#[derive(Debug, PartialEq)]
pub struct LexerError {
    pub index: usize,
    pub line: usize,
    pub col: usize,
    pub slice: String,
}

pub struct TokenLexer<'source> {
    lexer: Lexer<'source, TokenKind>,
}

impl TokenLexer<'_> {
    /// Create a new token parser.
    pub fn new<'source>(source: &'source str) -> TokenLexer<'source> {
        let lexer = TokenKind::lexer(source);
        TokenLexer { lexer }
    }

    /// Parse tokens from the source.
    pub fn parse<'source>(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(kind) = self.lexer.next() {
            // If encountered a lexing error, throw it
            if let TokenKind::Error = kind {
                return Err(LexerError {
                    index: self.lexer.span().start,
                    line: 0,
                    col: 0,
                    slice: self.lexer.slice().into(),
                });
            }
            // Else, push tokens to output
            tokens.push(Token {
                kind,
                index: self.lexer.span().start,
                len: self.lexer.span().len(),
                slice: self.lexer.slice().into(),
            });
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod token_lexer {
    use super::*;

    #[test]
    fn test_token_lexer() {
        let src = r#"
        fn main() {
            # assign variables
            let x = 1;
            let y = 2;
            let z = x + y;
        }"#;

        let mut lexer = TokenLexer::new(src);
        let tokens: Vec<TokenKind> = lexer.parse().unwrap().into_iter().map(|t| t.kind).collect();

        assert_eq!(
            tokens,
            vec![
                TokenKind::Keyword(Keyword::Fn),
                TokenKind::Ident,
                TokenKind::OpenParen,
                TokenKind::CloseParen,
                TokenKind::OpenBrace,
                TokenKind::LineComment,
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::Literal(LiteralKind::Int(Base::Decimal)),
                TokenKind::Semi,
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::Literal(LiteralKind::Int(Base::Decimal)),
                TokenKind::Semi,
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::Ident,
                TokenKind::Plus,
                TokenKind::Ident,
                TokenKind::Semi,
                TokenKind::CloseBrace
            ]
        )
    }

    #[test]
    fn test_token_lexer_error() {
        let src = "let hello_world = ℵ";
        let mut lexer = TokenLexer::new(src);
        let res = lexer.parse();

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            LexerError {
                index: 18,
                line: 0,
                col: 0,
                slice: "ℵ".into(),
            }
        )
    }
}
