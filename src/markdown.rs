//!
//! Markdown table generation
//!
use thiserror::Error;

/// Minimum size for the width of a single column in the markdown table
pub const MIN_COLUMN_WIDTH: usize = 3;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("Empty contents in table")]
    EmptyContents,
}

///
/// Convert a row of values into a markdown string.
///
fn values_to_markdown_row(values: &[String], col_widths: &[usize]) -> String {
    format!(
        "|{}|",
        values
            .iter()
            .enumerate()
            .map(|(i, value)| format!(" {:<width$} ", value, width = col_widths[i]))
            .collect::<Vec<String>>()
            .join("|")
    )
}

///
/// Repeat a single value to fill the width of each markdown column.
///
fn repeated_to_markdown_row(value: &str, col_widths: &[usize]) -> String {
    format!(
        "|{}|",
        col_widths
            .iter()
            .map(|&width| format!(" {} ", value.repeat(width)))
            .collect::<Vec<String>>()
            .join("|")
    )
}

///
/// Markdown table
///
#[derive(Debug)]
pub struct MarkdownTable {
    contents: Vec<Vec<String>>,
    has_header: bool,
}

impl MarkdownTable {
    ///
    /// Create a new markdown table from a 2D vector of data.
    ///
    pub fn new(contents: Vec<Vec<String>>) -> Self {
        MarkdownTable {
            contents,
            has_header: false,
        }
    }

    ///
    /// Set to true to use the first row of data as the header.
    ///
    pub fn has_header(&mut self, has_header: bool) -> &mut Self {
        self.has_header = has_header;
        self
    }

    ///
    /// Get the maximum number of columns needed to represent the table.
    ///
    fn get_num_columns(&self) -> usize {
        self.contents
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0)
    }

    ///
    /// Get a list of all the column widths based on the longest value in a column.
    ///
    fn get_column_widths(&self) -> Vec<usize> {
        let mut col_widths = vec![MIN_COLUMN_WIDTH; self.get_num_columns()];

        for line in self.contents.iter() {
            for (col_num, col_value) in line.iter().enumerate() {
                if col_value.len() > col_widths[col_num] {
                    col_widths[col_num] = col_value.len();
                };
            }
        }
        col_widths
    }

    ///
    /// Output the table as a markdown string
    ///
    pub fn to_markdown(&self) -> Result<String, MarkdownError> {
        let col_widths = self.get_column_widths();

        let heading_rendered = if self.has_header {
            let values = self.contents.first().ok_or(MarkdownError::EmptyContents)?;
            values_to_markdown_row(values, &col_widths)
        } else {
            repeated_to_markdown_row("?", &col_widths)
        };
        let separator = repeated_to_markdown_row("-", &col_widths);
        let contents_iter = if self.has_header {
            self.contents.iter().skip(1)
        } else {
            self.contents.iter().skip(0)
        };
        let contents_rendered = contents_iter
            .map(|row| values_to_markdown_row(row, &col_widths))
            .collect::<Vec<String>>()
            .join("\n");

        Ok(format!(
            "{}\n{}\n{}\n",
            heading_rendered, separator, contents_rendered
        ))
    }
}
