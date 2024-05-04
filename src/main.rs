//!
//! A text-to-table CLI tool.
//!
//! Reads a plain text based table and formats the table.
//!

#![allow(unused)]

use clap::Parser;
use std::process;

mod cli_args;
mod io;
mod lines;
mod markdown;
mod table;

fn main() {
    let args = crate::cli_args::CliArgs::parse();
    let delimiters_str = args
        .delimiters
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Format                : {}", args.format);
    println!("Input path            : {}", args.input.display());
    println!("Output path           : {}", args.output.display());
    println!("Delimiters            : {}", delimiters_str);
    println!("Contiguous delimiters : {}", args.contiguous_delimiters);
    println!("Quoted fields         : {}", args.quoted_fields);
    println!("Has header            : {}", args.has_header);

    let table = crate::io::read(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing input data: {}", err);
        process::exit(1);
    });
    crate::io::write(&args, table).unwrap_or_else(|err| {
        eprintln!("Problem writing output data: {}", err);
        process::exit(1);
    });
}
