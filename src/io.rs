use crate::cli_args::{CliArgs, TableOutputFmt};
use crate::markdown::MarkdownTable;
use crate::sqlddl::SqlDdlTable;
use crate::table::{Table, TableBuilder};
use build_html::{Html, Table as HtmlTable};
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

///
/// Primary entrypoint for reading a file, converting to a Table of Lines, parsing, and then
/// splitting records.
///
pub fn read(args: &CliArgs) -> Result<Table, Box<dyn Error>> {
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
pub fn write(args: &CliArgs, table: Table) -> Result<(), Box<dyn Error>> {
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
        TableOutputFmt::Md => {
            let md_table = MarkdownTable::new(contents)
                .has_header(args.has_header)
                .to_markdown()?;
            let mut file = File::create(&args.output)?;
            file.write_all(md_table.as_bytes())?;
        }
        TableOutputFmt::Html => {
            let html_table = HtmlTable::from(contents).to_html_string();
            let mut file = File::create(&args.output)?;
            file.write_all(html_table.as_bytes())?;
        }
        TableOutputFmt::Json => {}
        TableOutputFmt::Sql => {
            let sql_ddl_table = SqlDdlTable::new(contents)
                .has_header(args.has_header)
                .to_sql()?;
            let mut file = File::create(&args.output)?;
            file.write_all(sql_ddl_table.as_bytes())?;
        }
    }
    Ok(())
}
