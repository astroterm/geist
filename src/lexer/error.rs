#![allow(unused)]

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Diagnostic, Error)]
enum LexerError {
    #[error("the 'Geist-{attribute}' attribute is unrecognized or unsupported")]
    #[diagnostic(
        help("you might have a typo or be using an outdated version of geist"),
        code("geist::unknown_attribute"),
        severity(Error)
    )]
    UnknownAttributeError {
        attribute: String,

        #[label(primary, "invalid directive here")]
        span: SourceSpan
    },

    #[error("date is not formatted according to the ISO 8601 standard")]
    #[diagnostic(
        help("modify the date so it uses the 'YYYY-MM-DD' format"),
        code("geist::invalid_date"),
        severity(Error)
    )]
    InvalidDateError {
        date: String,

        #[label(primary, "invalid date here")]
        span: SourceSpan
    }
}