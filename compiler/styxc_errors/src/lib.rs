use snippet::SnippetKind;
use styxc_span::Span;
use thiserror::Error;
use crate::snippet::{Snippet, SnippetTitle};

mod snippet;

/// An enum of all possible errors thrown by the compiler.
#[derive(Debug, Error, Clone)]
pub enum ErrorKind {
	/// E0001 - Not Implemented
	#[error("E0001 - the feature `{0}` has not been implemented")]
	E0001(String),
}

/// A fatal error thrown by the compiler.
pub struct FatalError {
	/// The error kind.
	pub kind: ErrorKind,
	/// The span of the source file that caused this error.
	pub span: Span,
}

impl FatalError {
	/// Creates a new fatal error.
	pub fn new(span: Span, kind: ErrorKind) -> FatalError {
		FatalError { span, kind }
	}
}

impl From<FatalError> for Snippet {
	fn from(err: FatalError) -> Snippet {
		Snippet {
			title: SnippetTitle {
				message: err.kind.to_string(),
				kind: SnippetKind::Fatal
			},
			span: err.span
		}
	}
}
