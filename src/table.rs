//!
//! Table structures and functions
//!

use crate::lines::{
    Line, LineIgnoreContiguous, LineQuotedIgnoreContiguous, LineQuotedSplitContiguous,
    LineSplitContiguous, ParseError,
};
use std::error::Error;
use std::path::PathBuf;

///
/// Enum of all tables with different line types.
///
pub enum Table {
    SplitContiguous(TableContent<LineSplitContiguous>),
    IgnoreContiguous(TableContent<LineIgnoreContiguous>),
    QuotedSplitContiguous(TableContent<LineQuotedSplitContiguous>),
    QuotedIgnoreContiguous(TableContent<LineQuotedIgnoreContiguous>),
}

impl Table {
    pub fn split(&self) -> Result<Vec<Vec<String>>, ParseError> {
        match self {
            Table::SplitContiguous(t) => t.split(),
            Table::IgnoreContiguous(t) => t.split(),
            Table::QuotedSplitContiguous(t) => t.split(),
            Table::QuotedIgnoreContiguous(t) => t.split(),
        }
    }
}

///
/// Table builder. This builder generates a Table variant depending on the provided parameters.
///
pub struct TableBuilder {
    delimiters: Vec<char>,
    contiguous_delimiters: bool,
    quoted_fields: bool,
}

impl Default for TableBuilder {
    fn default() -> Self {
        TableBuilder {
            delimiters: vec![' '],
            contiguous_delimiters: false,
            quoted_fields: false,
        }
    }
}

impl TableBuilder {
    pub fn new() -> Self {
        TableBuilder::default()
    }

    pub fn delimiters(&mut self, delimiters: Vec<char>) -> &mut Self {
        self.delimiters = delimiters;
        self
    }

    pub fn contiguous_delimiters(&mut self, contiguous_delimiters: bool) -> &mut Self {
        self.contiguous_delimiters = contiguous_delimiters;
        self
    }

    pub fn quoted_fields(&mut self, quoted_fields: bool) -> &mut Self {
        self.quoted_fields = quoted_fields;
        self
    }

    pub fn from_path(&mut self, filepath: &PathBuf) -> Result<Table, Box<dyn Error>> {
        use Table::*;
        let contents = std::fs::read_to_string(filepath)?;
        let table = match (self.contiguous_delimiters, self.quoted_fields) {
            (false, false) => SplitContiguous(TableContent::new(&contents, &self.delimiters)),
            (true, false) => IgnoreContiguous(TableContent::new(&contents, &self.delimiters)),
            (false, true) => QuotedSplitContiguous(TableContent::new(&contents, &self.delimiters)),
            (true, true) => QuotedIgnoreContiguous(TableContent::new(&contents, &self.delimiters)),
        };
        Ok(table)
    }
}

///
/// Table that contains the vec of lines and the required line delimiters.
///
pub struct TableContent<T: Line> {
    lines: Vec<T>,
    delimiters: Vec<char>,
}

impl<T: Line> TableContent<T> {
    pub fn new(contents: &str, delimiters: &[char]) -> Self {
        TableContent {
            lines: contents.lines().map(T::new).collect(),
            delimiters: delimiters.to_vec(),
        }
    }

    pub fn split(&self) -> Result<Vec<Vec<String>>, ParseError> {
        let contents = self
            .lines
            .iter()
            .map(|line| line.split(&self.delimiters))
            .collect::<Result<Vec<Vec<String>>, _>>()?;

        Ok(contents)
    }
}
