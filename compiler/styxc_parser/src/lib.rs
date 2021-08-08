use styxc_ast::{Node, NodeKind};
use styxc_lexer::{Token, TokenKind};

#[derive(Debug)]
enum TokenParserError {}

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
    fn parse(mut self, tokens: Vec<Token>) -> Result<Node, TokenParserError> {
        // create the root ast node
        let root = Node {
            id: 0,
            kind: NodeKind::Root { children: vec![] },
        };

        let mut children = Vec::new();

        // iterate over tokens and parse
        for token in tokens {
            let next_node = Node {
                id: self.next,
                kind: NodeKind::Unknown
            };
            match token.kind {
                TokenKind::Ident => {
                    self.parse_ident_or_keyword(token);
                }
                TokenKind::Unknown => todo!(),
                TokenKind::KeywordLet => todo!(),
                TokenKind::KeywordConst => todo!(),
                TokenKind::KeywordFor => todo!(),
                TokenKind::KeywordWhile => todo!(),
                TokenKind::KeywordLoop => todo!(),
                TokenKind::KeywordBreak => todo!(),
                TokenKind::KeywordContinue => todo!(),
                TokenKind::KeywordFn => todo!(),
                TokenKind::KeywordAsync => todo!(),
                TokenKind::KeywordReturn => todo!(),
                TokenKind::KeywordAwait => todo!(),
                TokenKind::KeywordImport => todo!(),
                TokenKind::KeywordImportFrom => todo!(),
                TokenKind::KeywordType => todo!(),
                TokenKind::KeywordIf => todo!(),
                TokenKind::KeywordElse => todo!(),
                TokenKind::KeywordMatch => todo!(),
                TokenKind::KeywordEnum => todo!(),
                TokenKind::KeywordPublic => todo!(),
                TokenKind::KeywordPrivate => todo!(),
                TokenKind::KeywordProtected => todo!(),
                TokenKind::Whitespace => todo!(),
                TokenKind::LineComment => todo!(),
                TokenKind::BlockComment => todo!(),
                TokenKind::LiteralInt(_) => todo!(),
                TokenKind::LiteralFloat(_) => todo!(),
                TokenKind::LiteralChar => todo!(),
                TokenKind::LiteralString => todo!(),
                TokenKind::SymbolSemi => todo!(),
                TokenKind::SymbolOpenBrace => todo!(),
                TokenKind::SymbolCloseBrace => todo!(),
                TokenKind::SymbolOpenParen => todo!(),
                TokenKind::SymbolCloseParen => todo!(),
                TokenKind::SymbolOpenBracket => todo!(),
                TokenKind::SymbolCloseBracket => todo!(),
                TokenKind::SymbolPlus => todo!(),
                TokenKind::SymbolMinus => todo!(),
                TokenKind::SymbolStar => todo!(),
                TokenKind::SymbolSlash => todo!(),
                TokenKind::SymbolPercent => todo!(),
                TokenKind::SymbolEq => todo!(),
                TokenKind::SymbolNot => todo!(),
                TokenKind::SymbolAnd => todo!(),
                TokenKind::SymbolOr => todo!(),
                TokenKind::SymbolLt => todo!(),
                TokenKind::SymbolGt => todo!(),
                TokenKind::SymbolCaret => todo!(),
                TokenKind::SymbolTilde => todo!(),
                TokenKind::SymbolQuestion => todo!(),
                TokenKind::SymbolColon => todo!(),
                TokenKind::SymbolDot => todo!(),
                TokenKind::SymbolAt => todo!(),
            }
            children.push(next_node);
        }

        Ok(root)
    }

    /// Parse an identifier or keyword.
    fn parse_ident_or_keyword(&self, token: Token) -> () {
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
        let root = TokenParser::new().parse(tokens).unwrap();

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
