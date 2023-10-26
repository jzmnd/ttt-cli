//!
//! A text-to-table CLI tool.
//!
//! Reads a plain text based table and formats the table.
//!

#![allow(unused)]

use clap::Parser;

mod cli_args;
mod lines;
mod readfile;
mod table;

fn main() {
    let args = crate::cli_args::CliArgs::parse();
    let delimiters_str = args
        .delimiters
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Output                : {}", args.output);
    println!("Path                  : {}", args.filepath.display());
    println!("Delimiters            : {}", delimiters_str);
    println!("Contiguous delimiters : {}", args.contiguous_delimiters);
    println!("Quoted fields         : {}", args.quoted_fields);

    crate::readfile::read(&args);
}
