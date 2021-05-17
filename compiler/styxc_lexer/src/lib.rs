use std::iter::from_fn;

use logos::{Lexer, Logos};

/// Represents a token parsed by the lexer.
#[derive(Clone)]
pub struct Token {
    pub length: usize,
    pub token_type: TokenType,
}

fn integer_kilo(lex: &mut Lexer<TokenType>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
    Some(n * 1_000)
}

fn integer_mega(lex: &mut Lexer<TokenType>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
    Some(n * 1_000_000)
}

fn float_e_notation(lex: &mut Lexer<TokenType>) -> Option<f64> {
    let slice = lex.slice();
    let n: f64 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

/// An enum of possible token types.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    /// An identifier token.
    #[regex(r"[a-zA-Z_]+")]
    Ident,

    /// Represents the 'let' token.
    #[token("let")]
    DeclarationInitiator,

    /// Represents a constant initiator.
    #[token("const")]
    ConstantInitiator,

    /// Represents a function initiator.
    #[token("fn")]
    FuncInitiator,

    /// Represents an integer literal.
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    #[regex("-?[0-9]+k", integer_kilo)]
    #[regex("-?[0-9]+m", integer_mega)]
    Integer(i64),

    /// Represents a float literal.
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[0-9]+\.[0-9]+e[0-9]+", float_e_notation)]
    Float(f64),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lex = TokenType::lexer(input);
    from_fn(move || {
        let token = Token {
            token_type: lex.next().unwrap(),
            length: lex.slice().len()
        };
        Some(token)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lex = TokenType::lexer("hello_world");
        assert_eq!(lex.next(), Some(TokenType::Ident));
    }

    #[test]
    fn test_integer() {
        let mut lex = TokenType::lexer("1\n-30\n2k\n3m");
        assert_eq!(lex.next(), Some(TokenType::Integer(1)));
        assert_eq!(lex.next(), Some(TokenType::Integer(-30)));
        assert_eq!(lex.next(), Some(TokenType::Integer(2_000)));
        assert_eq!(lex.next(), Some(TokenType::Integer(3_000_000)));
    }

    #[test]
    fn test_float() {
        let mut lex = TokenType::lexer("1.0\n2e2\n3.1e3");
        assert_eq!(lex.next(), Some(TokenType::Float(1.0)));
    }

    #[test]
    fn test_tokenize() {
        let mut iter = tokenize("input\n123");
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        assert_eq!(first.length, "input".len());
        assert_eq!(first.token_type, TokenType::Ident);
        assert_eq!(second.length, "123".len());
        assert_eq!(second.token_type, TokenType::Integer(123));
    }
}
