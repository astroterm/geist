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
    }
}