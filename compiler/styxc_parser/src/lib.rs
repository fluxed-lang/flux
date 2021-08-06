use styxc_lexer::{Token, TokenKind};

#[derive(thiserror::Error, Debug)]
enum TokenParserError {}

struct TokenParser {
    tokens: Vec<Token>,
}

impl TokenParser {
    /// Create a new token parser with the target tokens.
    fn new(tokens: Vec<Token>) -> TokenParser {
        TokenParser { tokens }
    }

    /// Parse the tokens into a list of tokens.
    fn parse(self) -> Result<(), TokenParserError> {
        let mut tokens = self.tokens.iter();

        // iterate over tokens and parse
        while let Some(token) = tokens.next() {
            match token.kind {
                TokenKind::Ident => {
                    &self.parse_ident_or_keyword(token);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse an identifier or keyword.
    fn parse_ident_or_keyword(&self, token: &Token) -> () {
        match token.slice.as_str() {
            "let" => {}
            _ => {}
        }
    }
}
