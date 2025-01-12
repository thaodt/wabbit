//! Input handling and error context for the Wabbit compiler
//!
//! Manages:
//! - Source code input
//! - Error context extraction for meaningful error messages
//! - Source line formatting for error display

use crate::location::Span;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    pub extract: String,
    pub span: Span,
}

impl PartialEq for ErrorContext {
    fn eq(&self, other: &Self) -> bool {
        if self.span.is_empty() || other.span.is_empty() {
            return true;
        }
        self.extract == other.extract && self.span == other.span
    }
}

impl ErrorContext {
    pub fn new(input: &Input, span: Span) -> Self {
        if input.source.is_empty() {
            return Self {
                extract: "".to_string(),
                span,
            };
        }

        let extract: String = input
            .source
            .split('\n')
            .skip(span.start.line - 1)
            .take(span.end.line - span.start.line + 1)
            .enumerate()
            .fold(String::new(), |mut acc, (i, line)| {
                let cur_line = i + span.start.line;
                let start = if cur_line == span.start.line {
                    span.start.col
                } else {
                    1
                };
                let end = if cur_line == span.end.line {
                    span.end.col
                } else {
                    line.len()
                };

                let underline = " ".repeat(start - 1) + &"^".repeat(end - start + 1);
                write!(acc, "{:>4} | {}\n     | {}\n", cur_line, line, underline).unwrap();
                acc
            });

        Self { extract, span }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\n{}\n", self.extract)
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    pub source: &'a str,
}

impl<'a> Input<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }
}

impl AsRef<str> for Input<'_> {
    fn as_ref(&self) -> &str {
        self.source
    }
}
