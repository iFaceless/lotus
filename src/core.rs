//! Core implementation for html table converter.
use anyhow;
use scraper::{ElementRef, Html};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// Macro sel! parses css selector expression.
macro_rules! sel {
    ($x: expr) => {
        scraper::Selector::parse($x).unwrap()
    };
}

/// A table row contains multiple columns.
type Row = Vec<String>;

/// Table represents a table, wraps headers and rows.
#[derive(Debug)]
pub struct Table {
    rows: Vec<Row>,
    headers: Vec<String>,
}

impl Table {
    /// Create a new table object.
    fn new() -> Self {
        Table {
            rows: Vec::new(),
            headers: Vec::new(),
        }
    }

    /// Add a row to table.
    fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// Add a header to table.
    fn add_header(&mut self, hdr: String) {
        self.headers.push(hdr);
    }

    /// Render table object as markdown table.
    pub fn to_markdown(&self) -> String {
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
pub fn html_table_to_markdown_table<P: AsRef<Path>>(src: P) -> anyhow::Result<Vec<Table>> {
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

/// Parse a table element, collect headers and rows,
/// return an [`Table`] object.
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
