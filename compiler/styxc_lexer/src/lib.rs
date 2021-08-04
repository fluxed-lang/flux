use std::error::Error;

use logos::Logos;

#[derive(Debug, PartialEq)]
enum Base {
    Hexadecimal,
    Decimal,
    Octal,
    Binary
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
            return Base::Decimal
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
enum LiteralKind {
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

    #[regex("\"(\"|[^\"])*\"")]
    String
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
enum Token {
    #[error]
    Error,

    /// Represents an identifier or keyword.
    #[regex("[a-zA-Z_][a-zA-Z_0-9]+")]
    Ident,

    /// Represents a generic whitespace character. This includes tabs, spaces, and newlines.
    #[regex("\\s")]
    Whitespace,
    
    /// Represents a line comment.
    #[regex("#[^\n]+")]
    LineComment,

    /// Represents a block comment.
    #[regex("/\\*.*\\*/")]
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
    At
}

#[cfg(test)]
mod token {
    use logos::internal::LexerInternal;

    use super::*;

    #[test]
    fn test_ident() {
        let mut lexer = Token::lexer("hello_world i_like_foxes_123 xbox360");
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Ident));
    }

    #[test]
    fn test_line_comment() {
        let mut lexer = Token::lexer("# this is a comment\nhello_world");
        assert_eq!(lexer.next(), Some(Token::LineComment));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Ident));
    }

    #[test]
    fn test_block_comment() {
        let mut lexer = Token::lexer("/*this is a comment*/");
        assert_eq!(lexer.next(), Some(Token::BlockComment));
    }

    #[test]
    fn test_tokens() {
        let mut lexer = Token::lexer(";{}[]()+-*/%=!&|<>^~?:.@");

        assert_eq!(lexer.next(), Some(Token::Semi));
        assert_eq!(lexer.next(), Some(Token::OpenBrace));
        assert_eq!(lexer.next(), Some(Token::CloseBrace));
        assert_eq!(lexer.next(), Some(Token::OpenBracket));
        assert_eq!(lexer.next(), Some(Token::CloseBracket));
        assert_eq!(lexer.next(), Some(Token::OpenParen));
        assert_eq!(lexer.next(), Some(Token::CloseParen));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::Slash));
        assert_eq!(lexer.next(), Some(Token::Percent));
        assert_eq!(lexer.next(), Some(Token::Eq));
        assert_eq!(lexer.next(), Some(Token::Not));
        assert_eq!(lexer.next(), Some(Token::And));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::Lt));
        assert_eq!(lexer.next(), Some(Token::Gt));
        assert_eq!(lexer.next(), Some(Token::Caret));
        assert_eq!(lexer.next(), Some(Token::Tilde));
        assert_eq!(lexer.next(), Some(Token::Question));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Dot));
        assert_eq!(lexer.next(), Some(Token::At));
    }

    #[test]
    fn test_expression() {
        let mut lexer = Token::lexer("hello: int = 2;");
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Eq));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Literal(LiteralKind::Int(Base::Decimal))));
        assert_eq!(lexer.next(), Some(Token::Semi));
    }

    #[test]
    fn test_binary_expression() {
        let mut lexer = Token::lexer("1 + 2 * 3");
        assert_eq!(lexer.next(), Some(Token::Literal(LiteralKind::Int(Base::Decimal))));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Literal(LiteralKind::Int(Base::Decimal))));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        assert_eq!(lexer.next(), Some(Token::Literal(LiteralKind::Int(Base::Decimal))));
    }

    #[test]
    fn test_code() {
        let src = r#"
        fn main() {
            let x = 1;
            let y = 2;
            let z = x + y;
        }"#;

        let mut lexer = Token::lexer(src);

        // \n
        assert_eq!(lexer.next(), Some(Token::Whitespace));

        // indent
        for _ in 0..8 {
            assert_eq!(lexer.next(), Some(Token::Whitespace));
        }

        // fn
        assert_eq!(lexer.next(), Some(Token::Ident));
        // space
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        // main
        assert_eq!(lexer.next(), Some(Token::Ident));
        // ()
        assert_eq!(lexer.next(), Some(Token::OpenParen));
        assert_eq!(lexer.next(), Some(Token::CloseParen));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        // {
        assert_eq!(lexer.next(), Some(Token::OpenBrace));
        assert_eq!(lexer.next(), Some(Token::Whitespace));

        for _ in 0..12 {
            assert_eq!(lexer.next(), Some(Token::Whitespace));
        }

        // let
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        // x
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        // =
        assert_eq!(lexer.next(), Some(Token::Eq));
        assert_eq!(lexer.next(), Some(Token::Whitespace));
        // 1
        assert_eq!(lexer.next(), Some(Token::Literal(LiteralKind::Int(Base::Decimal))));
        assert_eq!(lexer.next(), Some(Token::Semi));
        
    }
}
