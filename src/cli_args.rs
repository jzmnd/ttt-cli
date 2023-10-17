//!
//! CLI argument parsing
//!

use clap::{Parser, ValueEnum};
use clap_verbosity_flag::Verbosity;
use std::fmt;
use std::path::PathBuf;

/// A text-to-table CLI tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The output table format
    #[arg(short, long, value_enum, default_value_t = TableOutputFmt::Csv)]
    pub output: TableOutputFmt,

    /// The file path to read
    #[arg(short, long)]
    pub filepath: PathBuf,

    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TableOutputFmt {
    /// Comma Separated Variables
    Csv,
    /// Markdown
    Md,
    /// HTML
    Html,
    /// JSON
    Json,
    /// SQL DDL file
    Sql,
}

impl fmt::Display for TableOutputFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TableOutputFmt::Csv => write!(f, "Comma Separated Variable (.csv)"),
            TableOutputFmt::Md => write!(f, "Markdown (.md)"),
            TableOutputFmt::Html => write!(f, "HTML (.html)"),
            TableOutputFmt::Json => write!(f, "JSON (.json)"),
            TableOutputFmt::Sql => write!(f, "SQL DDL file (.sql)"),
        }
    }
}
