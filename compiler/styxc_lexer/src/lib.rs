use logos::{Lexer, Logos};

#[derive(Debug, PartialEq)]
pub enum Base {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}

impl Base {
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

    /// The "enum" keyword, used for declaring an enumeration.
    #[token("enum")]
    KeywordEnum,

    #[token("public")]
    KeywordPublic,

    #[token("private")]
    KeywordPrivate,

    #[token("protected")]
    KeywordProtected,

    /// Represents a generic whitespace character. This includes tabs, spaces, and newlines.
    #[regex("\\s+", logos::skip)]
    Whitespace,

    /// Represents a line comment.
    #[regex("#[^\n]+")]
    LineComment,

    /// Represents a block comment.
    #[regex("/\\*[^*]*\\*/")]
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
    SymbolSemi,

    #[token("{")]
    SymbolOpenBrace,

    #[token("}")]
    SymbolCloseBrace,

    #[token("(")]
    SymbolOpenParen,

    #[token(")")]
    SymbolCloseParen,

    #[token("[")]
    SymbolOpenBracket,

    #[token("]")]
    SymbolCloseBracket,

    #[token("+")]
    SymbolPlus,

    #[token("-")]
    SymbolMinus,

    #[token("*")]
    SymbolStar,

    #[token("/")]
    SymbolSlash,

    #[token("%")]
    SymbolPercent,

    #[token("=")]
    SymbolEq,

    #[token("!")]
    SymbolNot,

    #[token("&")]
    SymbolAnd,

    #[token("|")]
    SymbolOr,

    #[token("<")]
    SymbolLt,

    #[token(">")]
    SymbolGt,

    #[token("^")]
    SymbolCaret,

    #[token("~")]
    SymbolTilde,

    #[token("?")]
    SymbolQuestion,

    #[token(":")]
    SymbolColon,

    #[token(".")]
    SymbolDot,

    #[token("@")]
    SymbolAt,
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
        let mut lexer = TokenKind::lexer("let const for while loop break continue fn async return await import from type if else match enum");
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
        let mut lexer = TokenKind::lexer("/**/ /*a*/ /* this is a comment */");
        assert_eq!(lexer.next(), Some(TokenKind::BlockComment));
        assert_eq!(lexer.next(), Some(TokenKind::BlockComment));
        assert_eq!(lexer.next(), Some(TokenKind::BlockComment));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_tokens() {
        let mut lexer = TokenKind::lexer(";{}[]()+-*/%=!&|<>^~?:.@");

        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOpenBrace));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCloseBrace));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOpenBracket));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCloseBracket));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOpenParen));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCloseParen));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolPlus));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolMinus));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolStar));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSlash));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolPercent));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolNot));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolAnd));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOr));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolLt));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolGt));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCaret));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolTilde));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolQuestion));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolColon));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolDot));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolAt));
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
        assert_eq!(lexer.next(), Some(TokenKind::SymbolColon));
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_binary_expression() {
        let mut lexer = TokenKind::lexer("1 + 2 * 3");
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolPlus));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolStar));
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
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));
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
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOpenParen));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCloseParen));
        // {
        assert_eq!(lexer.next(), Some(TokenKind::SymbolOpenBrace));

        // comment
        assert_eq!(lexer.next(), Some(TokenKind::LineComment));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        // x
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        // 1
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        // y
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        // 2
        assert_eq!(
            lexer.next(),
            Some(TokenKind::LiteralInt(Base::Decimal))
        );
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));

        // let
        assert_eq!(lexer.next(), Some(TokenKind::KeywordLet));
        // z
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // =
        assert_eq!(lexer.next(), Some(TokenKind::SymbolEq));
        // x
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        // +
        assert_eq!(lexer.next(), Some(TokenKind::SymbolPlus));
        // y
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.next(), Some(TokenKind::SymbolSemi));

        // } and EOF
        assert_eq!(lexer.next(), Some(TokenKind::SymbolCloseBrace));
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
                TokenKind::SymbolOpenParen,
                TokenKind::SymbolCloseParen,
                TokenKind::SymbolOpenBrace,
                TokenKind::LineComment,
                TokenKind::KeywordLet, 
                TokenKind::Ident,
                TokenKind::SymbolEq,
                TokenKind::LiteralInt(Base::Decimal),
                TokenKind::SymbolSemi,
                TokenKind::KeywordLet,
                TokenKind::Ident,
                TokenKind::SymbolEq,
                TokenKind::LiteralInt(Base::Decimal),
                TokenKind::SymbolSemi,
                TokenKind::KeywordLet,
                TokenKind::Ident,
                TokenKind::SymbolEq,
                TokenKind::Ident,
                TokenKind::SymbolPlus,
                TokenKind::Ident,
                TokenKind::SymbolSemi,
                TokenKind::SymbolCloseBrace
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
            TokenKind::SymbolEq,
            TokenKind::SymbolSemi
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
