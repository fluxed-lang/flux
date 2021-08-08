use std::{iter::Peekable, vec::IntoIter};

use styxc_ast::{Node, ExprKind};
use styxc_lexer::{Token, TokenKind};

#[derive(Debug)]
enum TokenParserError {
    UnexpectedEOI,
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind
    }
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
    pub fn parse(mut self, tokens: Vec<Token>) -> Result<Node, TokenParserError> {
        // create the root ast node
        let root = Node {
            id: 0,
            kind: ExprKind::Root { children: vec![] },
        };

        let mut children = Vec::new();
        let mut tokenIter = tokens.into_iter().peekable();

        // iterate over tokens and parse
        while let Some(token) = tokenIter.next() {
            let next_node = Node {
                id: self.next,
                kind: ExprKind::Unknown
            };
            match token.kind {
                TokenKind::KeywordLet => { self.parse_declaration(tokenIter); },
                _ => panic!("unexpected token {:?}", token.kind)
            }
            children.push(next_node);
        }

        Ok(root)
    }

    /// Require a token of the target kind.
    fn require_token(token: Option<Token>, kind: TokenKind) -> Result<(), TokenParserError> { 
        if !token.is_some() {
            return Err(TokenParserError::UnexpectedEOI);
        }
        let token = token.unwrap();
        if token.kind != kind {
            return Err(TokenParserError::ExpectedToken {
                expected: kind,
                found: token.kind
            });
        }
        Ok(())
    }

    fn declare_identifier(&mut self, identifier: String) {

    }

    fn declare_variable(&mut self) {

    }

    fn parse_declaration(&mut self, tokens: Peekable<IntoIter<Token>>) -> Result<Node, TokenParserError> {
        let ident = tokens.next();
        // parse identifier
        let ident = match Self::require_token(ident, TokenKind::Ident) {
            Ok(()) => ident.unwrap(),
            Err(e) => return Err(e.into())
        };
        
        // create ident ndoe
        let ident_node = Node {
            kind: ExprKind::Ident {
                id: 0,
                name: "".into()
            },
            id: self.next_id(),
        };


        // parse '='
        let eq = tokens.next();
        let eq = match Self::require_token(eq, TokenKind::SymbolEq) {
            Ok(()) => eq.unwrap(),
            Err(e) => return Err(e.into())
        }; 


        // create declaration node
        let decl_node = Node {
            id: self.next_id(),
            kind: ExprKind::Declaration {
                var: self.declare_variable(),
                value: 
            }
        }
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

        assert_eq!(
            root,
            Node {
                id: 0,
                kind: ExprKind::Root {
                    children: vec![Node {
                        id: 1,
                        kind: ExprKind::BinOp {
                            kind: BinOpKind::Assign,
                            lhs: Node {
                                id: 2,
                                kind: ExprKind::Ident {
                                    id: 0,
                                    name: "x".into(),
                                }
                            }
                            .into(),
                            rhs: Node {
                                id: 3,
                                kind: ExprKind::BinOp {
                                    kind: BinOpKind::Add,
                                    lhs: Node {
                                        id: 4,
                                        kind: ExprKind::BinOp {
                                            kind: BinOpKind::Mul,
                                            lhs: Node {
                                                id: 5,
                                                kind: ExprKind::Ident {
                                                    id: 1,
                                                    name: "y".into(),
                                                }
                                            }
                                            .into(),
                                            rhs: Node {
                                                id: 6,
                                                kind: ExprKind::Ident {
                                                    id: 2,
                                                    name: "z".into(),
                                                }
                                            }
                                            .into()
                                        }
                                    }
                                    .into(),
                                    rhs: Node {
                                        id: 7,
                                        kind: ExprKind::Literal(LiteralKind::Int(1)),
                                    }
                                    .into()
                                }
                            }
                            .into()
                        }
                    }]
                }
            }
        );
    }
}
