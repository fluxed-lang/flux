use std::{iter::Peekable, vec::IntoIter};

use styxc_ast::{Declaration, Ident, Mutability, Span, Stmt, StmtKind, AST};
use styxc_lexer::{Token, TokenKind};

#[derive(Debug)]
enum TokenParserError {
    UnexpectedEOI,
    ExpectedToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
    },
}

struct TokenParser {
    next: usize,
}

impl TokenParser {
    /// Create a new token parser with the target tokens.
    fn new() -> TokenParser {
        TokenParser { next: 0 }
    }

    fn next_id(&mut self) -> usize {
        self.next += 1;
        self.next
    }

    /// Parse the tokens into a list of tokens.
    pub fn parse(mut self, tokens: Vec<Token>) -> Result<AST, TokenParserError> {
        // create the root ast node
        let root = AST::new();
        let mut children = Vec::new();
        let mut tokenIter = tokens.into_iter().peekable();
        // iterate over tokens and parse
        while let Some(token) = tokenIter.next() {
            children.push(self.parse_stmt(&mut tokenIter)?);
        }
        Ok(root)
    }

    /// Require a token of the target kind.
    fn require_token(token: Option<Token>, kind: TokenKind) -> Result<Token, TokenParserError> {
        if !token.is_some() {
            return Err(TokenParserError::UnexpectedEOI);
        }
        let token = token.unwrap();
        if token.kind != kind {
            return Err(TokenParserError::ExpectedToken {
                expected: vec![kind],
                found: token.kind,
            });
        }
        Ok(token)
    }

    /// Parse a statement into an AST node.
    pub fn parse_stmt(
        &mut self,
        mut tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Stmt, TokenParserError> {
        let token = tokens.next();
        // ensure token exists
        if token.is_none() {
            return Err(TokenParserError::UnexpectedEOI);
        }
        // match the token kind
        let token = token.unwrap();
        match token.kind {
            TokenKind::KeywordLet => self.parse_declaration(tokens),
            _ => Err(TokenParserError::ExpectedToken {
                expected: vec![TokenKind::KeywordLet],
                found: token.kind,
            }),
        }
    }

    /// Attempt to parse a declaration.
    fn parse_declaration(
        &mut self,
        mut tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Stmt, TokenParserError> {
        let ident = tokens.next();
        // parse identifier
        let ident = match Self::require_token(ident, TokenKind::Ident) {
            Ok(token) => token,
            Err(e) => return Err(e.into()),
        };
        // create ident ndoe
        let ident = Stmt {
            id: self.next_id(),
            kind: StmtKind::Ident(Ident {
                id: self.next_id(),
                name: ident.slice,
                span: Span(ident.index, ident.index + ident.len),
            }),
        };
        // parse '='
        let eq = tokens.next();
        match Self::require_token(eq, TokenKind::SymbolEq) {
            Ok(_) => {}
            Err(e) => return Err(e.into()),
        };

        // attempt to parse a statement
        let value = self.parse_stmt(tokens)?;

        Ok(Stmt {
            id: ident.id,
            kind: StmtKind::Declaration(Declaration {
                ident: ident.into(),
                mutability: Mutability::Immutable,
                value: value.into(),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use styxc_ast::{BinOpKind, LiteralKind};
    use styxc_lexer::TokenLexer;

    #[test]
    fn test_bin_exp() {
        // parse source
        let tokens = TokenLexer::new("x = y * z + 1").parse().tokens;
        let root = TokenParser::new().parse(tokens).unwrap();
    }
}
