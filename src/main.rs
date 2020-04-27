//! A handful cli to convert simple html table to markdown table.
use anyhow;
use scraper::{ElementRef, Html};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// Macro sel! parses css selector expression
macro_rules! sel {
    ($x: expr) => {
        scraper::Selector::parse($x).unwrap()
    };
}

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = env!("CARGO_PKG_VERSION"))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// input html file with tables
    #[structopt(value_name = "INPUT", parse(from_os_str))]
    input: PathBuf,
    /// output file to save markdown table
    #[structopt(long, short, value_name = "OUTPUT", parse(from_os_str))]
    output: Option<PathBuf>,
    /// open output file automatically
    #[structopt(long)]
    open_after_converted: bool,
    /// text editor command
    #[structopt(long)]
    text_editor: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let out = html_table_to_markdown_table(opt.input)?
        .iter()
        .map(|t| t.to_markdown())
        .collect::<Vec<String>>()
        .join("\n");

    match opt.output {
        Some(pth) => {
            File::create(pth)?.write_all(out.as_bytes())?;
        }
        None => {
            println!("{}", out);
        }
    }

    Ok(())
}

type Row = Vec<String>;

#[derive(Debug)]
struct Table {
    rows: Vec<Row>,
    headers: Vec<String>,
}

impl Table {
    fn new() -> Self {
        Table {
            rows: Vec::new(),
            headers: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    fn add_header(&mut self, row: String) {
        self.headers.push(row);
    }

    fn to_markdown(&self) -> String {
        let mut lines = Vec::new();
        // add header
        lines.push(format!("|{}|", self.headers.join(" | ")));

        let sep = self
            .headers
            .iter()
            .map(|_| "---".to_owned())
            .collect::<Vec<String>>()
            .join(" | ");
        lines.push(format!("|{}|", sep));

        // add rows
        for row in self.rows.iter() {
            lines.push(format!("|{}|", row.join(" | ")));
        }

        lines.join("\n")
    }
}

/// Convert html table to markdown table.
fn html_table_to_markdown_table<P: AsRef<Path>>(src: P) -> anyhow::Result<Vec<Table>> {
    let mut rdr = BufReader::new(File::open(src)?);
    let mut content = String::new();
    rdr.read_to_string(&mut content)?;

    let fragment = Html::parse_fragment(content.as_ref());
    let selector = sel!("table");

    let mut tables: Vec<Table> = Vec::new();

    for table in fragment.select(&selector) {
        tables.push(parse_table(&table));
    }

    Ok(tables)
}

fn parse_table(table_element: &ElementRef) -> Table {
    let mut table = Table::new();

    let thead = sel!("thead th");

    for hd in table_element.select(&thead) {
        table.add_header(hd.text().collect::<String>());
    }

    let tr = sel!("tbody tr");
    let td = sel!("td");

    for row in table_element.select(&tr) {
        let values = row
            .select(&td)
            .map(|r| r.text().collect::<String>())
            .collect::<Vec<_>>();
        table.add_row(values);
    }

    table
}
