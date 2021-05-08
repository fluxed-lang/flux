use annotate_snippets::{display_list::{DisplayList, FormatOptions}, snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation}};
use peg::{error::ParseError, str::LineCol};

/// Convert a LineCol to a tuple containing the row and column position.
fn linecol_to_tuple(line_col: &LineCol) -> (usize, usize) {
    (
        line_col.line,
        line_col.column
    )
}

/// Turn a parse error into a snippet.
pub(crate) fn print_error(source: String, parse_error: &ParseError<LineCol>) {
    // format as error
    let str_error = &parse_error.to_string();
    // create snippet
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some(str_error),
            id: None,
            annotation_type: AnnotationType::Error
        }),
        footer: vec![],
        slices: vec![Slice {
           annotations: vec![SourceAnnotation {
               range: linecol_to_tuple(&parse_error.location),
               label: "",
               annotation_type: AnnotationType::Error
           }],
           fold: true,
           origin: None,
           line_start: parse_error.location.line,
           source: &source
        }],
        opt: FormatOptions {
            color: true,
            ..Default::default()
        },
    };
    // print snippet
    let dl = DisplayList::from(snippet);
    println!("{}", dl);
}
