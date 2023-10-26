//!
//! CLI argument parsing
//!

use clap::{Parser, ValueEnum};
use clap_verbosity_flag::Verbosity;
use std::fmt;
use std::path::PathBuf;

/// Arguments to the text-to-table CLI tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The output table format
    #[arg(short, long, value_enum, default_value_t = TableOutputFmt::Csv)]
    pub output: TableOutputFmt,

    /// The file path to read
    #[arg(short, long)]
    pub filepath: PathBuf,

    /// The delimiter to use
    #[arg(short, long, value_enum, default_values_t = [Delimiter::Space])]
    pub delimiters: Vec<Delimiter>,

    /// Whether to treat contiguous delimiter as a single delimiter
    #[arg(long)]
    pub contiguous_delimiters: bool,

    /// Whether to treat text in double-quotes as a single field
    #[arg(long)]
    pub quoted_fields: bool,

    #[clap(flatten)]
    pub verbose: Verbosity,
}

/// Possible table output formats
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

/// Possible text field delimiters
#[derive(Debug, Clone, ValueEnum)]
pub enum Delimiter {
    /// Space
    Space,
    /// Tab
    Tab,
    /// Comma (,)
    Comma,
    /// Vertical bar (|)
    Vbar,
    /// Period (.)
    Period,
    /// Colon (:)
    Colon,
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Delimiter::Space => write!(f, "Space"),
            Delimiter::Tab => write!(f, "Tab"),
            Delimiter::Comma => write!(f, "Comma"),
            Delimiter::Vbar => write!(f, "Vertical bar"),
            Delimiter::Period => write!(f, "Period"),
            Delimiter::Colon => write!(f, "Colon"),
        }
    }
}

impl Delimiter {
    pub fn as_char(&self) -> char {
        match self {
            Delimiter::Space => ' ',
            Delimiter::Tab => '\t',
            Delimiter::Comma => ',',
            Delimiter::Vbar => '|',
            Delimiter::Period => '.',
            Delimiter::Colon => ':',
        }
    }
}
