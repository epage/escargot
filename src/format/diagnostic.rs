//! This module contains `Diagnostic` and the types/functions it uses for deserialization.

use std::borrow;
use std::path;

type CowPath<'a> = borrow::Cow<'a, path::Path>;
type CowStr<'a> = borrow::Cow<'a, str>;

/// The error code associated to this diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCode<'a> {
    /// The code itself.
    #[serde(borrow)]
    pub code: CowStr<'a>,
    /// An explanation for the code
    #[serde(borrow)]
    pub explanation: Option<CowStr<'a>>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// A line of code associated with the Diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanLine<'a> {
    /// The line of code associated with the error
    #[serde(borrow)]
    pub text: CowStr<'a>,
    /// Start of the section of the line to highlight. 1-based, character offset in self.text
    pub highlight_start: usize,
    /// End of the section of the line to highlight. 1-based, character offset in self.text
    pub highlight_end: usize,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Macro expansion information associated with a diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanMacroExpansion<'a> {
    /// span where macro was applied to generate this code; note that
    /// this may itself derive from a macro (if
    /// `span.expansion.is_some()`)
    #[serde(borrow)]
    pub span: DiagnosticSpan<'a>,

    /// name of macro that was applied (e.g., "foo!" or "#[derive(Eq)]")
    #[serde(borrow)]
    pub macro_decl_name: CowStr<'a>,

    /// span where macro was defined (if known)
    #[serde(borrow)]
    pub def_site_span: Option<DiagnosticSpan<'a>>,

    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// A section of the source code associated with a Diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan<'a> {
    /// The file name this diagnostic comes from.
    #[serde(borrow)]
    pub file_name: CowPath<'a>,
    /// The byte offset in the file where this diagnostic starts from.
    pub byte_start: u32,
    /// The byte offset in the file where this diagnostic ends.
    pub byte_end: u32,
    /// 1-based. The line in the file.
    pub line_start: usize,
    /// 1-based. The line in the file.
    pub line_end: usize,
    /// 1-based, character offset.
    pub column_start: usize,
    /// 1-based, character offset.
    pub column_end: usize,
    /// Is this a "primary" span -- meaning the point, or one of the points,
    /// where the error occurred?
    pub is_primary: bool,
    /// Source text from the start of line_start to the end of line_end.
    #[serde(borrow)]
    pub text: Vec<DiagnosticSpanLine<'a>>,
    /// Label that should be placed at this location (if any)
    #[serde(borrow)]
    pub label: Option<CowStr<'a>>,
    /// If we are suggesting a replacement, this will contain text
    /// that should be sliced in atop this span.
    #[serde(borrow)]
    pub suggested_replacement: Option<CowStr<'a>>,
    /// If the suggestion is approximate
    pub suggestion_applicability: Option<Applicability>,
    /// Macro invocations that created the code at this span, if any.
    #[serde(borrow)]
    pub expansion: Option<Box<DiagnosticSpanMacroExpansion<'a>>>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Whether a suggestion can be safely applied.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Applicability {
    /// The suggested replacement can be applied automatically safely
    MachineApplicable,
    /// The suggested replacement has placeholders that will need to be manually
    /// replaced.
    HasPlaceholders,
    /// The suggested replacement may be incorrect in some circumstances. Needs
    /// human review.
    MaybeIncorrect,
    /// The suggested replacement will probably not work.
    Unspecified,
    #[doc(hidden)]
    #[serde(other)]
    Unknown,
}

/// A diagnostic message generated by rustc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic<'a> {
    /// The error message of this diagnostic.
    #[serde(borrow)]
    pub message: CowStr<'a>,
    /// The associated error code for this diagnostic
    #[serde(borrow)]
    pub code: Option<DiagnosticCode<'a>>,
    /// The severity of the diagnostic.
    pub level: DiagnosticLevel,
    /// A list of source code spans this diagnostic is associated with.
    #[serde(borrow)]
    pub spans: Vec<DiagnosticSpan<'a>>,
    /// Associated diagnostic messages.
    #[serde(borrow)]
    pub children: Vec<Diagnostic<'a>>,
    /// The message as rustc would render it
    #[serde(borrow)]
    pub rendered: Option<CowStr<'a>>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// The diagnostic level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticLevel {
    /// Internal compiler error
    #[serde(rename = "error: internal compiler error")]
    Ice,
    /// Error
    Error,
    /// Warning
    Warning,
    /// Note
    Note,
    /// Help
    Help,
    #[cfg(not(feature = "strict_unstable"))]
    #[doc(hidden)]
    #[serde(other)]
    Unknown,
}
