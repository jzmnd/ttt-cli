//!
//! SQL DDL generation
//!
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlDdlError {
    #[error("Empty contents in table")]
    EmptyContents,
}

///
/// SQL DDL table
///
#[derive(Debug)]
pub struct SqlDdlTable {
    contents: Vec<Vec<String>>,
    has_header: bool,
}

impl SqlDdlTable {
    ///
    /// Create a new SQL DDL table from a 2D vector of data.
    ///
    pub fn new(contents: Vec<Vec<String>>) -> Self {
        SqlDdlTable {
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
    /// Output the table as SQL DDL statement.
    ///
    pub fn to_sql(&self) -> Result<String, SqlDdlError> {
        let header_names = if self.has_header {
            self.contents
                .first()
                .ok_or(SqlDdlError::EmptyContents)?
                .clone()
        } else {
            vec!["?".to_string(); self.get_num_columns()]
        };
        let contents_iter = if self.has_header {
            self.contents.iter().skip(1)
        } else {
            self.contents.iter().skip(0)
        };
        let contents_rendered = contents_iter
            .map(|row| format!("('{}')", row.join("','")))
            .collect::<Vec<String>>()
            .join(",\n");

        Ok(format!(
            "INSERT INTO table_name\n({})\nVALUES\n{};\n",
            header_names.join(","),
            contents_rendered,
        ))
    }
}
