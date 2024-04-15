use crate::cli_args::{CliArgs, TableOutputFmt};
use crate::table::{TableBuilder, TableEnum};
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;

///
/// Primary entrypoint for reading a file, converting to a Table of Lines, parsing, and then
/// splitting records.
///
pub fn read(args: &CliArgs) -> Result<TableEnum, Box<dyn Error>> {
    let delimiters = args.delimiters.iter().map(|d| d.as_char()).collect();
    let table = TableBuilder::new()
        .quoted_fields(args.quoted_fields)
        .contiguous_delimiters(args.contiguous_delimiters)
        .delimiters(delimiters)
        .from_path(&args.input)?;

    Ok(table)
}

///
/// Write outputs to file in the specified format.
///
pub fn write(args: &CliArgs, table: TableEnum) -> Result<(), Box<dyn Error>> {
    let contents = table.split()?;
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
        TableOutputFmt::Md => {}
        TableOutputFmt::Html => {}
        TableOutputFmt::Json => {}
        TableOutputFmt::Sql => {}
    }
    Ok(())
}
