use crate::lines::Line;
use crate::lines::ParseError;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Table<T: Line> {
    lines: Vec<T>,
}

impl<T: Line> From<&str> for Table<T> {
    fn from(c: &str) -> Self {
        Table {
            lines: c.lines().map(T::new).collect(),
        }
    }
}

impl<T: Line> Table<T> {
    fn auto_column_count(&self, delimiters: &[char]) -> Result<usize, ParseError> {
        let num_fields = self.lines.iter().map(|a| a.num_fields(delimiters).unwrap());

        let freqs = num_fields.fold(HashMap::new(), |mut f, v| {
            *f.entry(v).or_insert(0) += 1;
            f
        });

        freqs
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(value, _)| value)
            .ok_or(ParseError::ColumnCountError)
    }
}

pub fn read(filepath: &PathBuf) -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string(filepath);
    contents
}

// pub fn parse(contents: &str) {
//     for line in contents.lines() {
//         let line = Line>::from(line);
//     }
// }
