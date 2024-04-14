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
pub enum TableEnum {
    SplitContiguous(Table<LineSplitContiguous>),
    IgnoreContiguous(Table<LineIgnoreContiguous>),
    QuotedSplitContiguous(Table<LineQuotedSplitContiguous>),
    QuotedIgnoreContiguous(Table<LineQuotedIgnoreContiguous>),
}

impl TableEnum {
    pub fn split(&self) -> Result<Vec<Vec<String>>, ParseError> {
        match self {
            TableEnum::SplitContiguous(t) => t.split(),
            TableEnum::IgnoreContiguous(t) => t.split(),
            TableEnum::QuotedSplitContiguous(t) => t.split(),
            TableEnum::QuotedIgnoreContiguous(t) => t.split(),
        }
    }
}

///
/// Table builder. This builder generates a TableEnum variant depending on the provided parameters.
///
pub struct TableBuilder {
    delimiters: Vec<char>,
    contiguous_delimiters: bool,
    quoted_fields: bool,
    table: Option<TableEnum>,
}

impl Default for TableBuilder {
    fn default() -> Self {
        TableBuilder {
            delimiters: vec![' '],
            contiguous_delimiters: false,
            quoted_fields: false,
            table: None,
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

    pub fn from_path(&mut self, filepath: &PathBuf) -> Result<TableEnum, Box<dyn Error>> {
        use TableEnum::*;
        let contents = std::fs::read_to_string(filepath)?;
        let table = match (self.contiguous_delimiters, self.quoted_fields) {
            (false, false) => SplitContiguous(Table::new(&contents, &self.delimiters)),
            (true, false) => IgnoreContiguous(Table::new(&contents, &self.delimiters)),
            (false, true) => QuotedSplitContiguous(Table::new(&contents, &self.delimiters)),
            (true, true) => QuotedIgnoreContiguous(Table::new(&contents, &self.delimiters)),
        };
        Ok(table)
    }
}

///
/// Table that contains the vec of lines and the required line delimiters.
///
pub struct Table<T: Line> {
    lines: Vec<T>,
    delimiters: Vec<char>,
}

impl<T: Line> Table<T> {
    pub fn new(contents: &str, delimiters: &[char]) -> Self {
        Table {
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
