use crate::cli_args::{CliArgs, TableOutputFmt};
use crate::table::TableBuilder;
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;

///
/// Primary entrypoint for reading a file, converting to a Table of Lines, parsing, and then
/// splitting records.
///
pub fn read(args: &CliArgs) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let delimiters = args.delimiters.iter().map(|d| d.as_char()).collect();
    let table_reader = TableBuilder::new()
        .quoted_fields(args.quoted_fields)
        .contiguous_delimiters(args.contiguous_delimiters)
        .delimiters(delimiters)
        .from_path(&args.input)?;

    let contents = table_reader.split()?;
    Ok(contents)
}

///
/// Write outputs to file in the specified format.
///
pub fn write(args: &CliArgs, contents: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    match args.format {
        TableOutputFmt::Csv => {
            let mut wtr = WriterBuilder::new()
                .flexible(true)
                .has_headers(false)
                .from_path(&args.output)?;
            for result in contents.into_iter() {
                wtr.write_record(&result)?;
            }
            wtr.flush()?;
        }
        _ => todo!(),
    }
    Ok(())
}
