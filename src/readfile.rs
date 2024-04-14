use crate::cli_args::CliArgs;
use crate::table::TableBuilder;
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;

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
        .from_path(&args.input)?;

    let csv_contents = table_reader.to_csv()?;

    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .quoting(args.quoted_fields)
        .from_reader(csv_contents.as_bytes());

    let mut wtr = WriterBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(&args.output)?;
    for result in csv_reader.records() {
        let record = result?;
        wtr.write_record(&record)?;
        println!("{:?}", record);
    }
    wtr.flush()?;

    Ok(())
}
