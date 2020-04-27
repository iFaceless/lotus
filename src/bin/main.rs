//! A handful cli to convert simple html table to markdown table.
use lotus::html_table_to_markdown_table;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// input html file with tables
    #[structopt(value_name = "INPUT", parse(from_os_str))]
    input: PathBuf,
    /// output file to save markdown table (default to stdout)
    #[structopt(long, short, value_name = "OUTPUT", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let out = html_table_to_markdown_table(opt.input)?
        .iter()
        .map(|t| t.to_markdown() + "\n")
        .collect::<Vec<String>>()
        .join("\n");

    match opt.output {
        Some(pth) => {
            File::create(&pth)?.write_all(out.as_bytes())?;
        }
        None => {
            println!("{}", out);
        }
    }

    Ok(())
}
