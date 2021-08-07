use std::borrow::BorrowMut;

use styxc_ast::{Node, NodeKind};
use styxc_lexer::{Token, TokenKind};

#[derive(thiserror::Error, Debug)]
enum TokenParserError {}

struct TokenParser {
    tokens: Vec<Token>,
    next: usize,
}

impl TokenParser {
    /// Create a new token parser with the target tokens.
    fn new(tokens: Vec<Token>) -> TokenParser {
        TokenParser { tokens, next: 0 }
    }

    fn next_id(&mut self) -> usize {
        self.next += 1;
        self.next
    }

    /// Parse the tokens into a list of tokens.
    fn parse(&mut self) -> Result<Node, TokenParserError> {
        let mut tokens = self.tokens.iter();

        // create the root ast node
        let root = Node {
            id: 0,
            kind: NodeKind::Root { children: vec![] },
        };

        let mut children = Vec::new();

        // iterate over tokens and parse
        while let Some(token) = tokens.next() {
            let next_node = Node {
                id: self.next_id(),
                kind: NodeKind::Unknown
            };
            match token.kind {
                TokenKind::Ident => {
                    &self.parse_ident_or_keyword(token);
                }
                _ => {}
            }
            children.push(next_node);
        }

        Ok(root)
    }

    /// Parse an identifier or keyword.
    fn parse_ident_or_keyword(&self, token: &Token) -> () {
        match token.slice.as_str() {
            "let" => {}
            _ => {}
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
        let root = TokenParser::new(tokens).parse().unwrap();

        assert_eq!(
            root,
            Node {
                id: 0,
                kind: NodeKind::Root {
                    children: vec![Node {
                        id: 1,
                        kind: NodeKind::BinOp {
                            kind: BinOpKind::Assign,
                            lhs: Node {
                                id: 2,
                                kind: NodeKind::Ident {
                                    id: 0,
                                    name: "x".into(),
                                }
                            }
                            .into(),
                            rhs: Node {
                                id: 3,
                                kind: NodeKind::BinOp {
                                    kind: BinOpKind::Add,
                                    lhs: Node {
                                        id: 4,
                                        kind: NodeKind::BinOp {
                                            kind: BinOpKind::Mul,
                                            lhs: Node {
                                                id: 5,
                                                kind: NodeKind::Ident {
                                                    id: 1,
                                                    name: "y".into(),
                                                }
                                            }
                                            .into(),
                                            rhs: Node {
                                                id: 6,
                                                kind: NodeKind::Ident {
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
                                        kind: NodeKind::Literal(LiteralKind::Int(1)),
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
