use crate::cli_args::CliArgs;
use crate::lines::{
    Line, LineIgnoreContiguous, LineQuotedIgnoreContiguous, LineQuotedSplitContiguous,
    LineSplitContiguous, ParseError,
};
use csv::ReaderBuilder;
use std::collections::HashMap;
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
    fn to_csv(&self) -> Result<String, ParseError> {
        let csv_contents = match self {
            TableEnum::SplitContiguous(t) => t.to_csv()?,
            TableEnum::IgnoreContiguous(t) => t.to_csv()?,
            TableEnum::QuotedSplitContiguous(t) => t.to_csv()?,
            TableEnum::QuotedIgnoreContiguous(t) => t.to_csv()?,
        };
        Ok(csv_contents)
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
    fn new() -> Self {
        TableBuilder::default()
    }

    fn delimiters(&mut self, delimiters: Vec<char>) -> &mut Self {
        self.delimiters = delimiters;
        self
    }

    fn contiguous_delimiters(&mut self, contiguous_delimiters: bool) -> &mut Self {
        self.contiguous_delimiters = contiguous_delimiters;
        self
    }

    fn quoted_fields(&mut self, quoted_fields: bool) -> &mut Self {
        self.quoted_fields = quoted_fields;
        self
    }

    fn from_path(&mut self, filepath: &PathBuf) -> Result<TableEnum, Box<dyn Error>> {
        use TableEnum::*;
        let contents = std::fs::read_to_string(filepath)?;
        let table = match (self.contiguous_delimiters, self.quoted_fields) {
            (false, false) => SplitContiguous(Table::<LineSplitContiguous>::new(
                &contents,
                &self.delimiters,
            )),
            (true, false) => IgnoreContiguous(Table::<LineIgnoreContiguous>::new(
                &contents,
                &self.delimiters,
            )),
            (false, true) => QuotedSplitContiguous(Table::<LineQuotedSplitContiguous>::new(
                &contents,
                &self.delimiters,
            )),
            (true, true) => QuotedIgnoreContiguous(Table::<LineQuotedIgnoreContiguous>::new(
                &contents,
                &self.delimiters,
            )),
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
    fn new(contents: &str, delimiters: &[char]) -> Self {
        Table {
            lines: contents.lines().map(T::new).collect(),
            delimiters: delimiters.to_vec(),
        }
    }

    fn to_csv(&self) -> Result<String, ParseError> {
        let csv_contents = self
            .lines
            .iter()
            .map(|line| line.to_csv(&self.delimiters))
            .collect::<Result<Vec<String>, _>>()?
            .join("\n");

        Ok(csv_contents)
    }
}

///
/// Primary entrypoint for reading a file, converting to a Table of Lines, parsing, and then
/// converting to CSV records.
///
pub fn read(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let delimiters = args.delimiters.iter().map(|d| d.as_char()).collect();
    let table_reader = TableBuilder::new()
        .quoted_fields(args.quoted_fields)
        .contiguous_delimiters(args.contiguous_delimiters)
        .delimiters(delimiters)
        .from_path(&args.filepath)?;

    let csv_contents = table_reader.to_csv()?;

    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .quoting(args.quoted_fields)
        .from_reader(csv_contents.as_bytes());

    for result in csv_reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
