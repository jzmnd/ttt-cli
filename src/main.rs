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

fn main() {
    let args = crate::cli_args::CliArgs::parse();

    println!("Output : {}", args.output);
    println!("Path   : {}", args.filepath.display());

    let contents = crate::readfile::read(&args.filepath);
}
