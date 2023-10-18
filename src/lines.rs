//!
//! Text processing and parsing for individual lines
//!

use core::mem;
use thiserror::Error;

pub const DOUBLE_QUOTE: char = '\"';

///
/// Line parsing related errors
///
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cannot parse line")]
    CannotParseLine,
    #[error("Error auto-counting columns")]
    ColumnCountError,
}

///
/// Trait for lines that can be split into separate fields
///
pub trait Line {
    fn split(&self, delimiters: &[char]) -> Result<Vec<String>, ParseError>;
    fn num_fields(&self, delimiters: &[char]) -> Result<usize, ParseError> {
        Ok(self.split(delimiters)?.len())
    }
    fn new(line: &str) -> Self;
}

///
/// No double-quoted fields. Contiguous delimiters are treated separately.
///
#[derive(Debug)]
pub struct LineSplitContiguous {
    line: String,
}

impl Line for LineSplitContiguous {
    fn split(&self, delimiters: &[char]) -> Result<Vec<String>, ParseError> {
        Ok(self
            .line
            .split(delimiters)
            .map(String::from)
            .collect())
    }

    fn new(line: &str) -> Self {
        LineSplitContiguous {
            line: line.to_string(),
        }
    }
}

///
/// No double-quoted fields. Contiguous delimiters are treated as a single delimiter.
///
#[derive(Debug)]
pub struct LineIgnoreContiguous {
    line: String,
}

impl Line for LineIgnoreContiguous {
    fn split(&self, delimiters: &[char]) -> Result<Vec<String>, ParseError> {
        Ok(self
            .line
            .split(delimiters)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect())
    }

    fn new(line: &str) -> Self {
        LineIgnoreContiguous {
            line: line.to_string(),
        }
    }
}

enum CharState {
    /// Character is a delimiter
    Delimiter,
    /// Character is within an unquoted field
    Unquoted,
    /// Character is within a double quoted field
    Quoted,
}

///
/// Double-quoted fields allowed. Contiguous delimiters are treated separately.
///
#[derive(Debug)]
pub struct LineQuotedSplitContiguous {
    line: String,
}

impl Line for LineQuotedSplitContiguous {
    fn split(&self, delimiters: &[char]) -> Result<Vec<String>, ParseError> {
        let mut fields = Vec::new();
        let mut field = String::new();
        let mut chars = self.line.chars();
        let mut state = CharState::Delimiter;

        loop {
            let c = chars.next();
            state = match state {
                CharState::Delimiter => match c {
                    None => break,
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Quoted
                    }
                    Some(c) if delimiters.contains(&c) => {
                        fields.push(String::new());
                        CharState::Delimiter
                    }
                    Some(c) => {
                        field.push(c);
                        CharState::Unquoted
                    }
                },
                CharState::Unquoted => match c {
                    None => {
                        fields.push(mem::replace(&mut field, String::new()));
                        break;
                    }
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Quoted
                    }
                    Some(c) if delimiters.contains(&c) => {
                        fields.push(mem::replace(&mut field, String::new()));
                        CharState::Delimiter
                    }
                    Some(c) => {
                        field.push(c);
                        CharState::Unquoted
                    }
                },
                CharState::Quoted => match c {
                    None => return Err(ParseError::CannotParseLine),
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Unquoted
                    }
                    Some(c) => {
                        field.push(c);
                        CharState::Quoted
                    }
                },
            }
        }
        Ok(fields)
    }

    fn new(line: &str) -> Self {
        LineQuotedSplitContiguous {
            line: line.to_string(),
        }
    }
}

///
/// Double-quoted fields allowed. Contiguous delimiters are treated as a single delimiter.
///
#[derive(Debug)]
pub struct LineQuotedIgnoreContiguous {
    line: String,
}

impl Line for LineQuotedIgnoreContiguous {
    fn split(&self, delimiters: &[char]) -> Result<Vec<String>, ParseError> {
        let mut fields = Vec::new();
        let mut field = String::new();
        let mut chars = self.line.chars();
        let mut state = CharState::Delimiter;

        loop {
            let c = chars.next();
            state = match state {
                CharState::Delimiter => match c {
                    None => break,
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Quoted
                    }
                    Some(c) if delimiters.contains(&c) => CharState::Delimiter,
                    Some(c) => {
                        field.push(c);
                        CharState::Unquoted
                    }
                },
                CharState::Unquoted => match c {
                    None => {
                        fields.push(mem::replace(&mut field, String::new()));
                        break;
                    }
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Quoted
                    }
                    Some(c) if delimiters.contains(&c) => {
                        fields.push(mem::replace(&mut field, String::new()));
                        CharState::Delimiter
                    }
                    Some(c) => {
                        field.push(c);
                        CharState::Unquoted
                    }
                },
                CharState::Quoted => match c {
                    None => return Err(ParseError::CannotParseLine),
                    Some(DOUBLE_QUOTE) => {
                        field.push(DOUBLE_QUOTE);
                        CharState::Unquoted
                    }
                    Some(c) => {
                        field.push(c);
                        CharState::Quoted
                    }
                },
            }
        }
        Ok(fields)
    }

    fn new(line: &str) -> Self {
        LineQuotedIgnoreContiguous {
            line: line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_split_space_comma() {
        let s = r#"asdklsaj,,,alskjd,"kas  jd",,ksjd,sk,d"#;

        let delims = &[' ', ','];
        let ln1 = LineSplitContiguous::new(s);
        let ln2 = LineIgnoreContiguous::new(s);
        let ln3 = LineQuotedSplitContiguous::new(s);
        let ln4 = LineQuotedIgnoreContiguous::new(s);

        assert_eq!(ln1.num_fields(delims).unwrap(), 11);
        assert_eq!(ln2.num_fields(delims).unwrap(), 7);
        assert_eq!(ln3.num_fields(delims).unwrap(), 9);
        assert_eq!(ln4.num_fields(delims).unwrap(), 6);
    }

    #[test]
    fn test_line_split_comma() {
        let s = r#"asdklsaj,,,alskjd,"kas  jd",,ksjd,sk,d"#;

        let delims = &[','];
        let ln1 = LineSplitContiguous::new(s);
        let ln2 = LineIgnoreContiguous::new(s);
        let ln3 = LineQuotedSplitContiguous::new(s);
        let ln4 = LineQuotedIgnoreContiguous::new(s);

        assert_eq!(ln1.num_fields(delims).unwrap(), 9);
        assert_eq!(ln2.num_fields(delims).unwrap(), 6);
        assert_eq!(ln3.num_fields(delims).unwrap(), 9);
        assert_eq!(ln4.num_fields(delims).unwrap(), 6);
    }
}
