use logos::{Lexer, Logos};

/// A trait designed to be implemented by enums representing
/// simple string-based tokens.
trait Lexable {
    fn lex(src: &str) -> Self;
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Private,
    Protected,
    Public,
}

impl Lexable for Visibility {
    fn lex(src: &str) -> Self {
        use Visibility::*;
        match src {
            "private" => Private,
            "protected" => Protected,
            "public" => Public,
            _ => panic!("attempted to lex {} which is not a visibility modifier", src),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum Keyword {
    #[error]
    Error,

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

impl Lexable for Base {
    /// Parse the target slice into a string.
    fn lex(src: &str) -> Base {
        let mut slice = src.clone();
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
            _ => Base::Decimal
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[error]
    Unknown,

    /// Represents an identifier.
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Ident,

    ///
    /// KEYWORDS
    ///
    
    /// The "let" token, used in declarations.
    #[token("let")]
    KeywordLet,

    /// The "const" token, used in constant declarations.
    #[token("const")]
    KeywordConst,

    /// The "for" keyword, used for declaring loops over iterators or ranges.
    #[token("for")]
    KeywordFor,

    /// The "while" keyword, used for declaring a conditional loop.
    #[token("while")]
    KeywordWhile,

    /// The "loop" keyword, used for declaring a unconditional loop.
    #[token("loop")]
    KeywordLoop,

    /// The "break" keyword, used for breaking out of loops.
    #[token("break")]
    KeywordBreak,

    /// The "continue" keyword, used for continuing to the next iteration of a loop.
    #[token("continue")]
    KeywordContinue,

    /// The "fn" keyword, used for declaring functions.
    #[token("fn")]
    KeywordFn,

    /// The "async" keyword, used to declare an asynchronous function.
    #[token("async")]
    KeywordAsync,

    /// The "return" keyword, used for returning from a function.
    #[token("return")]
    KeywordReturn,

    /// The "await" keyword, used to wait for the current asynchronous operation to finish.
    #[token("await")]
    KeywordAwait,

    /// The "import" keyword, used to import external modules.
    #[token("import")]
    KeywordImport,

    /// The "from" keyword, used when declaring an aliased or destructing import.
    #[token("from")]
    KeywordImportFrom,

    /// The "type" keyword, used to declare a new type.
    #[token("type")]
    KeywordType,

    /// The "if" keyword, used for declaring conditional statements.
    #[token("if")]
    KeywordIf,

    /// The "else" keyword, used for declaring an "else" clause.
    #[token("else")]
    KeywordElse,

    /// The "match" keyword, used for declaring a pattern match.
    #[token("match")]
    KeywordMatch,

    /// The "try" keyword, used for declaring a try/catch block.
    #[token("try")]
    KeywordTry,

    /// The "catch" keyword, used for declaring a catch block.
    #[token("catch")]
    KeywordCatch,

    /// The "finally" keyword, used for declaring a finally block.
    #[token("finally")]
    KeywordFinally,

    /// The "enum" keyword, used for declaring an enumeration.
    #[token("enum")]
    KeywordEnum,

    #[token("public", |lex| Visibility::lex(lex.slice()))]
    #[token("private", |lex| Visibility::lex(lex.slice()))]
    #[token("protected", |lex| Visibility::lex(lex.slice()))]
    KeywordVisibility(Visibility),

    /// Represents a generic whitespace character. This includes tabs, spaces, and newlines.
    #[regex("\\s+", logos::skip)]
    Whitespace,

    /// Represents a line comment.
    #[regex("#[^\n]+")]
    LineComment,

    /// Represents a block comment.
    #[regex(r#"/\*(.|\n)*\*/"#)]
    BlockComment,

    ///
    /// LITERALS
    ///

    /// Represents any integer literal and its base.
    /// Matches both raw ints and integers with their base specified, e.g. 1234, or 0x1fff.
    #[regex("[+-]?[0-9]+", |lex| Base::lex(lex.slice()))]
    #[regex("[+-]?0x[0-9a-fA-F]+", |lex| Base::lex(lex.slice()) )]
    #[regex("[+-]?0d[0-9]+", |lex| Base::lex(lex.slice()) )]
    #[regex("[+-]?0o[0-7]+", |lex| Base::lex(lex.slice()) )]
    #[regex("[+-]?0b[01]+", |lex| Base::lex(lex.slice()) )]
    LiteralInt(Base),

    /// Represents any floating point literal. Matches both floating point and scientific notation.
    /// e.g. 0.1, 1e-10, 1.0e-10, 1.0e+10, 1.0e10, 1.0e-10
    #[regex("[+-]?[0-9]*\\.[0-9]+", |lex| Base::lex(lex.slice()))]
    #[regex("[+-]?[0-9]+e[+-]?[0-9]+", |lex| Base::lex(lex.slice()))]
    LiteralFloat(Base),

    #[regex("'.'")]
    #[regex(r#"'\\u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]'"#)]
    LiteralChar,

    #[regex("\"([^\"]|(\\\\\"))*\"")]
    LiteralString,

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
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordConst));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordFor));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordWhile));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLoop));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordBreak));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordContinue));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordFn));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordAsync));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordReturn));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordAwait));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordImport));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordImportFrom));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordType));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordIf));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordElse));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordMatch));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordTry));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordCatch));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordFinally));
        assert_eq!(lexer.next(), Some(TokenKind::KeywordEnum));
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
    fn test_int() {
        let mut lexer = TokenKind::lexer("1234");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralInt(Base::Decimal)));
    }

    #[test]
    fn test_int_hex() {
        let mut lexer = TokenKind::lexer("0x1234");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralInt(Base::Hexadecimal)));
    }

    #[test]
    fn test_int_oct() {
        let mut lexer = TokenKind::lexer("0o1234");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralInt(Base::Octal)));
    }

    #[test]
    fn test_int_bin() {
        let mut lexer = TokenKind::lexer("0b10100");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralInt(Base::Binary)));
    }

    #[test]
    fn test_float() {
        let mut lexer = TokenKind::lexer("12.34");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralFloat(Base::Decimal)));
    }

    #[test]
    fn test_float_exp() {
        let mut lexer = TokenKind::lexer("12.34e-5");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralFloat(Base::Decimal)));
        let mut lexer = TokenKind::lexer("-432e+10");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralFloat(Base::Decimal)));
    }

    #[test]
    fn test_char() {
        let mut lexer = TokenKind::lexer("'a'");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralChar));
    }

    #[test]
    fn test_unicode_char() {
        let mut lexer = TokenKind::lexer("'\\u1234'");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralChar));
    }

    #[test]
    fn test_string() {
        let mut lexer = TokenKind::lexer("\"foo\"");
        assert_eq!(lexer.next(), Some(TokenKind::LiteralString));
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
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_binary_expression() {
        let mut lexer = TokenKind::lexer("1 + 2 * 3");
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Plus));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Star));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_assignment() {
        let mut lexer = TokenKind::lexer("let x = 2;");
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
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
        assert_eq!(lexer.next(), Some(TokenKind::KeywordFn));
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
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        // x
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        // 1
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        // y
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::Eq));
        // 2
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::Semi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
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

pub struct LexerResult {
    /// A list of tokens that were successfully lexed. This does not include unknown tokens.
    pub tokens: Vec<Token>,
    /// A list of lexer errors that occurred. These correspond to unknown tokens.
    pub errors: Vec<LexerError>
}

impl LexerResult {
    /// Returns true if this lexer result contains errors.
    pub fn has_errs(&self) -> bool {
        self.errors.len() > 0
    }
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
    pub fn parse<'source>(&mut self) -> LexerResult {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(kind) = self.lexer.next() {
            // If encountered a lexing error, throw it
            if let TokenKind::Unknown = kind {
                errors.push(LexerError {
                    index: self.lexer.span().start,
                    line: 0,
                    col: 0,
                    slice: self.lexer.slice().into(),
                });
            } else {
                // Else, push tokens to output
                tokens.push(Token {
                    kind,
                    index: self.lexer.span().start,
                    len: self.lexer.span().len(),
                    slice: self.lexer.slice().into(),
                });
            }
        }
        
        LexerResult { tokens, errors }
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
        let tokens: Vec<TokenKind> = lexer.parse().tokens.into_iter().map(|t| t.kind).collect();

        assert_eq!(
            tokens,
            vec![
                TokenKind::KeywordFn,
                TokenKind::Ident,
                TokenKind::OpenParen,
                TokenKind::CloseParen,
                TokenKind::OpenBrace,
                TokenKind::LineComment,
                TokenKind::KeywordLet, 
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::LiteralInt(Base::Decimal),
                TokenKind::Semi,
                TokenKind::KeywordLet,
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::LiteralInt(Base::Decimal),
                TokenKind::Semi,
                TokenKind::KeywordLet,
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
        let src = "let hello_world = ℵ;";
        let mut lexer = TokenLexer::new(src);
        let res = lexer.parse();

        assert!(res.has_errs());

        let kinds: Vec<TokenKind> = res.tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(kinds, vec![
            TokenKind::KeywordLet,
            TokenKind::Ident,
            TokenKind::Eq,
            TokenKind::Semi
        ]);

        assert_eq!(
            res.errors[0],
            LexerError {
                index: 18,
                line: 0,
                col: 0,
                slice: "ℵ".into(),
            }
        );
    }
}
