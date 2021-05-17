use logos::{Lexer, Logos};

/// Represents a token parsed by the lexer.
struct Token {
    pub length: usize,
    pub token_type: TokenType,
}

fn integer_kilo(lex: &mut Lexer<TokenType>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
    Some(n * 1_000)
}

fn integer_mega(lex: &mut Lexer<TokenType>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
    Some(n * 1_000_000)
}

fn float_e_notation(lex: &mut Lexer<TokenType>) -> Option<f64> {
    let slice = lex.slice();
    let n: f64 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

/// An enum of possible token types.
#[derive(Logos, Debug, PartialEq)]
enum TokenType {
    /// An identifier token.
    #[regex(r"[a-zA-Z_]+")]
    Ident,

    /// Represents the 'let' token.
    #[token("let")]
    Declaration,

    /// Represents an integer literal.
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    #[regex("[0-9]+k", integer_kilo)]
    #[regex("[0-9]+m", integer_mega)]
    Integer(u64),

    /// Represents a float literal.
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[0-9]+\.[0-9]+e[0-9]+", float_e_notation)]
    Float(f64),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
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
        let mut lex = TokenType::lexer("1\n2k\n3m");
        assert_eq!(lex.next(), Some(TokenType::Integer(1)));
        assert_eq!(lex.next(), Some(TokenType::Integer(2_000)));
        assert_eq!(lex.next(), Some(TokenType::Integer(3_000_000)));
    }

    #[test]
    fn test_float() {
        let mut lex = TokenType::lexer("1.0\n2e2\n3.1e3");
        assert_eq!(lex.next(), Some(TokenType::Float(1.0)));
    }
}
