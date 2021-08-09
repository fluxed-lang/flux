use std::{iter::Peekable, vec::IntoIter};

use styxc_ast::{Declaration, Ident, Mutability, Span, Stmt, StmtKind, AST};
use styxc_lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum TokenParserError {
    /// Parser reached the end of input without having completed parsing.
    UnexpectedEOI,
    /// Parser encountered an unexpected token.
    UnexpectedToken {
        position: Span,
        expected: Vec<TokenKind>,
        found: TokenKind,
    },
}

pub struct TokenParser {
    next: usize,
}

impl TokenParser {
    /// Create a new token parser with the target tokens.
    pub fn new() -> TokenParser {
        TokenParser { next: 0 }
    }

    /// Increment the current AST node ID.
    fn next_id(&mut self) -> usize {
        self.next += 1;
        self.next
    }

    /// Parse the tokens into a list of tokens.
    pub fn parse(mut self, tokens: Vec<Token>) -> Result<AST, TokenParserError> {
        // create the root ast node
        let root = AST::new();
        let mut children = Vec::new();
        let mut tokens = tokens.into_iter().peekable();
        // iterate over tokens and parse
        while let Some(_) = tokens.peek() {
            children.push(self.parse_stmt(&mut tokens)?);
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
            return Err(TokenParserError::UnexpectedToken {
                position: Span(token.index, token.index + token.len),
                expected: vec![kind],
                found: token.kind,
            });
        }
        Ok(token)
    }

    /// Parse a statement into an AST node.
    pub fn parse_stmt(
        &mut self,
        tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Stmt, TokenParserError> {
        let token = tokens.peek();
        // ensure token exists
        if token.is_none() {
            return Err(TokenParserError::UnexpectedEOI);
        }
        // match the token kind
        let token = token.unwrap();
        match token.kind {
            TokenKind::KeywordLet => self.parse_declaration(tokens),
            TokenKind::Ident => self.parse_ident(tokens),
            _ => Err(TokenParserError::UnexpectedToken {
                position: Span(token.index, token.index + token.len),
                expected: vec![TokenKind::KeywordLet],
                found: token.kind,
            }),
        }
    }

    /// Attempt to parse an identifier.
    fn parse_ident(&mut self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, TokenParserError> { 
        Self::require_token(tokens.next(), TokenKind::Ident).map(|token| Stmt {
            id: self.next_id(),
            kind: StmtKind::Ident(Ident {
                 id: self.next_id(),
                name: token.slice,
                span: Span(token.index, token.index + token.len),
            }),
        })
    }

    /// Attempt to parse a declaration.
    fn parse_declaration(
        &mut self,
        tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Stmt, TokenParserError> {
        // parse let
        Self::require_token(tokens.next(), TokenKind::KeywordLet)?;
        // parse identifier
        let ident = self.parse_ident(tokens)?;
        // parse '='
        Self::require_token(tokens.next(), TokenKind::SymbolEq)?;
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
        let tokens = TokenLexer::new("let x = y * z + 1").parse().tokens;
        let root = TokenParser::new().parse(tokens).unwrap();

        assert_eq!(root.stmts, vec![]);
    }
}
