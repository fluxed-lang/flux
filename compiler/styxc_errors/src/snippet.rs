use styxc_span::Span;

/// Small implementation in the style of annotate-snippets.

/// The kind of snippet.
pub enum SnippetKind {
	/// A warning snippet.
	Warning,
	/// An error snippet.
	Error,
	/// A fatal error snippet.
	Fatal
}

pub struct SnippetTitle {
	/// The message of this snippet.
	pub message: String,
	/// The kind of this snippet.
	pub kind: SnippetKind
}

/// Represents an annotated snippet of source code.
pub struct Snippet {
	/// The title of this snippet.
	pub title: SnippetTitle,
	/// The span of the source code this snippet is annotating.
	pub span: Span
}
