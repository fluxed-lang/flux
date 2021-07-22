extern crate pest;
#[macro_use]
extern crate pest_derive;

use logos::{Lexer, Logos};
use std::{error::Error, str::Chars};

mod parser;

fn parse_char(lexer: &mut Lexer<Token>) -> Result<char, Box<dyn Error>> {
    // remove surrounding quotes
    let mut chars = lexer.slice().chars();
    chars.next();
    chars.next_back();
    let res = chars.as_str();

    println!("{}", res);

    match res.parse() {
        Ok(char) => Ok(char),
        Err(e) => Err(e.into()),
    }
}

fn extract_base_or_default(input: &str) -> Base {
    let mut chars = input.chars();
    if chars.next().unwrap() == '0' {
        return match chars.next() {
            Some('b') => Base::Binary,
            Some('x') => Base::Hexadecimal,
            Some('o') => Base::Octal,
            // int is just a 0
            None => {
                return Base::Decimal;
            }
            // default to decimal
            _ => Base::Decimal,
        };
    }
    Base::Decimal
}

/// Represents an integer literal in the token stream.
/// Holds information about its base prefix, and whether this is an invalid 'empty' integer.
#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    base: Base,
    empty_int: bool,
}

fn parse_integer(lexer: &mut Lexer<Token>) -> IntegerLiteral {
    let mut chars: Chars = lexer.slice().chars();
    // if starts with a base char
    let mut base = Base::Decimal;
    // okay to unwrap, must have matched at least one character
    if chars.next().unwrap() == '0' {
        base = match chars.next() {
            Some('b') => Base::Binary,
            Some('x') => Base::Hexadecimal,
            Some('o') => Base::Octal,
            // int is just a 0
            None => {
                return IntegerLiteral {
                    base: Base::Decimal,
                    empty_int: false,
                }
            }
            // default to decimal
            _ => Base::Decimal,
        };
    }
    // if the next char is a digit, otherwise it is just a base prefix
    match chars.next() {
        None => IntegerLiteral {
            base,
            empty_int: true,
        },
        Some(_) => IntegerLiteral {
            base,
            empty_int: false,
        },
    }
}

/// Represents a float literal in the token stream.
/// Holds information about its base prefix, and whether the exponent of the float is 'empty'.
#[derive(Debug, PartialEq)]
pub struct FloatLiteral {
    base: Base,
    empty_exponent: bool,
}
fn parse_float(lexer: &mut Lexer<Token>) -> FloatLiteral {
    let mut chars: Chars = lexer.slice().chars();
    // if starts with a base char
    let mut base = Base::Decimal;
    // okay to unwrap, must have matched at least one character
    if chars.next().unwrap() == '0' {
        base = match chars.next() {
            Some('b') => Base::Binary,
            Some('x') => Base::Hexadecimal,
            Some('o') => Base::Octal,
            // int is just a 0
            None => {
                return FloatLiteral {
                    base: Base::Decimal,
                    empty_exponent: true,
                }
            }
            // default to decimal
            _ => Base::Decimal,
        };
    }
    let mut found_exponent = false;
    loop {
        if let Some(x) = chars.next() {
            if x == 'e' && chars.next().is_some() {
                found_exponent = true;
                break;
            }
            continue;
        }
        break;
    }

    FloatLiteral {
        base,
        empty_exponent: !found_exponent,
    }
}

/// Represents the base of a float or integer literal.
#[derive(Debug, PartialEq)]
pub enum Base {
    Binary,
    Octal,
    Hexadecimal,
    Decimal,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]+")]
    Ident,

    #[regex(r"-?[0-9]+", |lex| parse_integer(lex))]
    #[regex(r"-?0[bdox][0-9a-fA-F]*", |lex| parse_integer(lex))]
    Integer(IntegerLiteral),

    #[regex(r"(-?[0-9]+)((\.[0-9]*)(e-?[0-9]+)?|(e-?[0-9]+))", |lex| parse_float(lex))]
    #[regex(r"-?0[bdox][0-9a-fA-F]+\.[0-9a-fA-F]*", |lex| parse_float(lex))]
    Float(FloatLiteral),

    #[regex(r#""."?"#)]
    String,

    // #[regex(r"'.+'", |lex| parse_char(lex))]
    // Char,
    #[regex(r"//.*")]
    LineComment,

    #[regex(r"/\*\s?(.*)\s?\*/")]
    BlockComment,

    // One-char tokens:
    /// ";"
    #[token(";")]
    Semi,
    /// ","
    #[token(",")]
    Comma,
    /// "."
    #[token(".")]
    Dot,
    /// "("
    #[token("(")]
    OpenParen,
    /// ")"
    #[token(")")]
    CloseParen,
    /// "{"
    #[token("{")]
    OpenBrace,
    /// "}"
    #[token("}")]
    CloseBrace,
    /// "["
    #[token("[")]
    OpenBracket,
    /// "]"
    #[token("]")]
    CloseBracket,
    /// "@"
    #[token("@")]
    At,
    /// "#"
    #[token("#")]
    Pound,
    /// "~"
    #[token("~")]
    Tilde,
    /// "?"
    #[token("?")]
    Question,
    /// ":"
    #[token(":")]
    Colon,
    /// "$"
    #[token("$")]
    Dollar,
    /// "="
    #[token("=")]
    Eq,
    /// "!"
    #[token("!")]
    Bang,
    /// "<"
    #[token("<")]
    Lt,
    /// ">"
    #[token(">")]
    Gt,
    /// "-"
    #[token("-")]
    Minus,
    /// "&"
    #[token("&")]
    And,
    /// "|"
    #[token("|")]
    Or,
    /// "+"
    #[token("+")]
    Plus,
    /// "*"
    #[token("*")]
    Star,
    /// "/"
    #[token("/")]
    Slash,
    /// "^"
    #[token("^")]
    Caret,
    /// "%"
    #[token("%")]
    Percent,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lexer = Token::lexer("hello_world;\n 1234");
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.next(), Some(Token::Semi));
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Decimal,
                empty_int: false
            }))
        );
    }

    #[test]
    fn test_integer_literal() {
        let mut lexer = Token::lexer("1234 -4321 0x3f 0b10101 0o70 0x");
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Decimal,
                empty_int: false
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Decimal,
                empty_int: false
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Hexadecimal,
                empty_int: false
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Binary,
                empty_int: false
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Octal,
                empty_int: false
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Integer(IntegerLiteral {
                base: Base::Hexadecimal,
                empty_int: true
            }))
        );
    }

    #[test]
    fn test_float_literal() {
        let mut lexer = Token::lexer("1.0 1. 0x1.f 10e3");
        assert_eq!(
            lexer.next(),
            Some(Token::Float(FloatLiteral {
                base: Base::Decimal,
                empty_exponent: true
            }))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::Float(FloatLiteral {
                base: Base::Decimal,
                empty_exponent: true
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Float(FloatLiteral {
                base: Base::Hexadecimal,
                empty_exponent: true
            }))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::Float(FloatLiteral {
                base: Base::Decimal,
                empty_exponent: false
            }))
        );
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Token::lexer("\"hello\n\"; \"world\n\";");
        assert_eq!(lexer.next(), Some(Token::String));
        assert_eq!(lexer.next(), Some(Token::Semi));
        assert_eq!(lexer.next(), Some(Token::String));
    }

    // #[test]
    // fn test_char_literal() {
    //     let mut lexer = Token::lexer("'a'");
    //     assert_eq!(lexer.next(), Some(Token::Char('a')));
    // }

    #[test]
    fn test_line_comment() {
        let mut lexer = Token::lexer("// this is a comment");
        assert_eq!(lexer.next(), Some(Token::LineComment));
    }

    #[test]
    fn test_line_comment_with_token() {
        let mut lexer = Token::lexer("// this is a comment\n;");
        assert_eq!(lexer.next(), Some(Token::LineComment));
        assert_eq!(lexer.next(), Some(Token::Semi));
    }

    #[test]
    fn test_block_comment() {
        let mut lexer = Token::lexer("/* this is a comment */");
        assert_eq!(lexer.next(), Some(Token::BlockComment));
    }

    #[test]
    fn test_single_char_tokens() {
        let mut lexer = Token::lexer(";,.(){}[]@#~?:$=!<>-&|+*/^%");
        assert_eq!(lexer.next(), Some(Token::Semi));
        assert_eq!(lexer.next(), Some(Token::Comma));
        assert_eq!(lexer.next(), Some(Token::Dot));
        assert_eq!(lexer.next(), Some(Token::OpenParen));
        assert_eq!(lexer.next(), Some(Token::CloseParen));
        assert_eq!(lexer.next(), Some(Token::OpenBrace));
        assert_eq!(lexer.next(), Some(Token::CloseBrace));
        assert_eq!(lexer.next(), Some(Token::OpenBracket));
        assert_eq!(lexer.next(), Some(Token::CloseBracket));
        assert_eq!(lexer.next(), Some(Token::At));
        assert_eq!(lexer.next(), Some(Token::Pound));
        assert_eq!(lexer.next(), Some(Token::Tilde));
        assert_eq!(lexer.next(), Some(Token::Question));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Dollar));
        assert_eq!(lexer.next(), Some(Token::Eq));
        assert_eq!(lexer.next(), Some(Token::Bang));
        assert_eq!(lexer.next(), Some(Token::Lt));
        assert_eq!(lexer.next(), Some(Token::Gt));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::And));
        assert_eq!(lexer.next(), Some(Token::Or));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::Slash));
        assert_eq!(lexer.next(), Some(Token::Caret));
        assert_eq!(lexer.next(), Some(Token::Percent));
    }
}
