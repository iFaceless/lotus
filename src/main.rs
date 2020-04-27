//! A handful cli to convert simple html table to markdown table.
use structopt::StructOpt;
use scraper::Html;
use anyhow;
use std::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// input html file with tables
    #[structopt(value_name = "INPUT")]
    input: String,
    /// output file to save markdown table
    #[structopt(long, short, value_name = "OUTPUT")]
    output: Option<String>,
    /// open output file automatically
    #[structopt(long)]
    open_after_converted: bool,
    /// text editor command
    #[structopt(long)]
    text_editor: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    dbg!(opt);
    Ok(())
}

/// Convert html table to markdown table.
fn html_table_to_markdown_table(input: String) -> anyhow::Result<String> {
    Ok("".to_owned())
}