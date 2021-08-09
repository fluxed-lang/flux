use styxc_ast::{Declaration, Ident, Mutability, Span, Stmt, StmtKind, AST};
use styxc_lexer::{Token, TokenKind};


/// Trait that allows iterators to step backwards.
pub trait BackwardIterator : Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub trait TinyPeekable : BackwardIterator {
    /// Returns the next token without advancing the iterator.
    fn peek(&mut self) -> Option<Self::Item>;
    /// Returns the previous token without advancing the iterator.
    fn peek_prev(&mut self) -> Option<Self::Item>;
}

/// A stream of tokens that can step forwards and backwards.
pub struct TokenStream<'a> {
    pos: Option<usize>,
    tokens: &'a Vec<Token>,
}

impl<'a> TokenStream<'a> {
    /// Create a new token stream from a vector of tokens.
    pub fn new(tokens: &'a Vec<Token>) -> TokenStream<'a> {
        TokenStream { pos: None, tokens }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<&'a Token> {
        let index = 
            match self.pos {
                Some(i) => i + 1,
                None => 0
            };

        self.pos = Some(index);
        self.tokens.get(index)
    }
}

impl<'a> BackwardIterator for TokenStream<'a> {
    fn prev(&mut self) -> Option<&'a Token> {
        let index = 
            match self.pos {
                Some(0) | None => return None,
                Some(i) => i - 1
            };

        self.pos = Some(index);
        self.tokens.get(index)
    }
}

impl <'a> TinyPeekable for TokenStream<'a> {
    fn peek(&mut self) -> Option<&'a Token> {
        let index = match self.pos {
            Some(i) => i + 1,
            None => 0
        };
        self.tokens.get(index + 1)
    }

    fn peek_prev(&mut self) -> Option<&'a Token> {
        let index = match self.pos {
            Some(0) | None => return None,
            Some(i) => i - 1
        };
        self.tokens.get(index)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenParserError {
    /// Parser reached the end of input without having completed parsing.
    UnexpectedEOI,
    /// Parser encountered an unexpected token.
    UnexpectedToken {
        position: Span,
        expected: Vec<TokenKind>,
        found: TokenKind,
    },
    // Parser encountered a mismatching delimiter.
    MismatchedDelimiter(Span)
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
        // check delimiters before parsing
        Self::check_delimitiers(&tokens)?;
        // create the root ast node
        let root = AST::new();
        let mut children = Vec::new();
        let mut stream = TokenStream::new(&tokens);
        // iterate over tokens and parse
        while let Some(_) = stream.peek() {
            children.push(self.parse_stmt(&mut stream)?);
        }
        Ok(root)
    }

    /// Check delimiters are correct.
    ///
    /// Before the TokenParser builds the AST, it performs a preliminary check to ensure delimiters are correct.
    /// It does this by iterating over all tokens, and keeping track of the most recent iterator in an iterator
    /// stack: a vector of deliminators with their kinds and open/close state recorded. By checking whether
    /// the last item on the stack is the same as the current deliminator when a closing deliminter is encountered,
    /// the exact location of any mismatching delimiters can be determined.
    ///
    /// The downside of this method is that it can only locate one mismatching delimiter at a time, but I believe
    /// it is impossible to do anything else without sacrificing knowledge of the delimiters whereabouts.
    fn check_delimitiers(tokens: &Vec<Token>) -> Result<(), TokenParserError> {
        /// An enum containing delimiter kind.
        #[derive(PartialEq)]
        enum DelimiterKind {
            Paren,
            Bracket,
            Brace
        }   

        /// A delimiter in the token stream.
        struct Delimiter(DelimiterKind, bool);
    
        impl Delimiter {
            /// Fetch a delimiter from a token kind.
            fn from_kind(kind: TokenKind) -> Option<Delimiter> {
                use DelimiterKind::*;
                match kind {
                    TokenKind::SymbolOpenParen => Some(Delimiter(Paren, true)),
                    TokenKind::SymbolOpenBracket => Some(Delimiter(Bracket, true)),
                    TokenKind::SymbolOpenBrace => Some(Delimiter(Brace, true)),
                    TokenKind::SymbolCloseParen => Some(Delimiter(Paren, false)),
                    TokenKind::SymbolCloseBracket => Some(Delimiter(Bracket, false)),
                    TokenKind::SymbolCloseBrace => Some(Delimiter(Brace, false)),
                    _ => None
                }
            }
        }
        // create a stack of delimiters.
        let mut stack: Vec<Delimiter> = vec![];
        // iterate over tokens and check delimiters
        for token in tokens {
            let delimiter = Delimiter::from_kind(token.kind);
            if delimiter.is_none() {
                continue;
            }
            let delimiter = delimiter.unwrap();
            // pop from the stack
            if !delimiter.1 {
                // check if there was no last delimiter - encountered a closing delimiter without an opening one.
                if stack.last().is_none() {
                    return Err(TokenParserError::MismatchedDelimiter(Span(token.index, token.index + token.len)));
                }
                // check if the last delimiter was of a different kind.
                let prev = stack.pop().unwrap();
                if prev.0 != delimiter.0 {
                    return Err(TokenParserError::MismatchedDelimiter(Span(token.index, token.index + token.len)));
                }
                // delimiter is okay, can continue.
            } else {
                // push to the stack
                stack.push(delimiter);
            }
        }
        
        Ok(())
    }

    /// Require a token of the target kind.
    fn require_token(token: Option<&Token>, kind: TokenKind) -> Result<&Token, TokenParserError> {
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
        stream: &mut TokenStream,
    ) -> Result<Stmt, TokenParserError> {
        let token = stream.peek();
        // ensure token exists
        if token.is_none() {
            return Err(TokenParserError::UnexpectedEOI);
        }
        // match the token kind
        let token = token.unwrap();
        match token.kind {
            TokenKind::KeywordLet => self.parse_declaration(stream),
            TokenKind::Ident => self.parse_ident(stream),
            _ => Err(TokenParserError::UnexpectedToken {
                position: Span(token.index, token.index + token.len),
                expected: vec![TokenKind::KeywordLet],
                found: token.kind,
            }),
        }
    }

    /// Peek the iterator and test if there is a binary operation.
    fn peek_bin_op(&mut self, stream: &mut TokenStream) -> Option<Stmt> {
        let token = stream.peek()?;
        // if token is not a binary operation symbol
        if !matches!(token.kind, TokenKind::SymbolStar | TokenKind::SymbolSlash | TokenKind::SymbolPlus | TokenKind::SymbolMinus) {
            return None;
        }
        // check if the previous token.
        let prev = stream.peek_prev()?;
        

        None
    }

    /// Attempt to parse an identifier.
    fn parse_ident(&mut self, stream: &mut TokenStream) -> Result<Stmt, TokenParserError> { 
        Self::require_token(stream.next(), TokenKind::Ident).map(|token| Stmt {
            id: self.next_id(),
            kind: StmtKind::Ident(Ident {
                 id: self.next_id(),
                name: token.slice.clone(),
                span: Span(token.index, token.index + token.len),
            }),
        })
    }

    /// Attempt to parse a declaration.
    fn parse_declaration(
        &mut self,
        stream: &mut TokenStream,
    ) -> Result<Stmt, TokenParserError> {
        // parse let
        Self::require_token(stream.next(), TokenKind::KeywordLet)?;
        // parse identifier
        let ident = self.parse_ident(stream)?;
        // parse '='
        Self::require_token(stream.next(), TokenKind::SymbolEq)?;
        // attempt to parse a statement
        let value = self.parse_stmt(stream)?;
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
    fn test_delimiters() {
        let tokens = TokenLexer::new("())").parse().tokens;
        let res = TokenParser::new().parse(tokens);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), TokenParserError::MismatchedDelimiter(Span(2, 3)));

        let tokens = TokenLexer::new("(]").parse().tokens;
        let res = TokenParser::new().parse(tokens);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), TokenParserError::MismatchedDelimiter(Span(1, 2)));
    }

    #[test]
    fn test_bin_exp() {
        // parse source
        let tokens = TokenLexer::new("let x = y * z + 1").parse().tokens;
        let root = TokenParser::new().parse(tokens).unwrap();

        assert_eq!(root.stmts, vec![]);
    }
}
