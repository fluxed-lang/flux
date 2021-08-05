use annotate_snippets::{display_list::{DisplayList, FormatOptions}, snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation}};
use thiserror::Error;
use styxc_lexer::LexerError;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("[E001] syntax error: unknown token '{:?}'", .0.slice)]
    UnknownToken(LexerError)
}

impl ErrorKind {
    fn report(self, source: &str) {
        match self {
            ErrorKind::UnknownToken(lexer_error) => {
                let title = format!("syntax error: unknown token '{}'", lexer_error.slice);
                let snippet = Snippet {
                    title: Some(Annotation {
                        label: Some(title.as_str()),
                        id: Some("E001"),
                        annotation_type: AnnotationType::Error
                    }),
                    slices: vec![
                        Slice {
                            source,
                            fold: true,
                            line_start: 0,
                            origin: None,
                            annotations: vec![
                                SourceAnnotation {
                                    label: "",
                                    annotation_type: AnnotationType::Error,
                                    range: (lexer_error.index, lexer_error.index + lexer_error.slice.chars().count())
                                }
                            ]
                        }
                    ],
                    footer: vec![],
                    opt: FormatOptions {
                        color: true,
                        ..Default::default()
                    },
                };
                let dl = DisplayList::from(snippet);
                println!("{}", dl);
            }
        }
    }
}

/// Report any lexing errors to stdout.
pub fn report_lexer_errors(source: &str, errors: Vec<LexerError>) {
    for e in errors.into_iter() {
        ErrorKind::UnknownToken(e).report(source)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use styxc_lexer::*;

    #[test]
    fn test_error() {
        let res = TokenLexer::new("let x = ℵ; x += 2; ℵ").parse();
        report_lexer_errors("let x = ℵ; x += 2; ℵ", res.errors);
    }
}