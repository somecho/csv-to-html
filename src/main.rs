use build_html::{Html, Table};
use clap::Parser;
use std::fs::{self, File};

/// Convert CSV table to HTML table
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// path to input file
    #[arg(short, long)]
    filename: String,
    /// the CSV file has no header
    #[arg(short, long)]
    no_header: bool,
    /// path of output file. If not specified, will print to stdout
    #[arg(short, long)]
    output: Option<String>,
}

fn csv_to_html(csv_reader: &mut csv::Reader<File>, has_header: &bool) -> String {
    let mut table = Table::new();
    match csv_reader.headers() {
        Ok(record) => {
            let header: Vec<&str> = record.iter().collect();
            if *has_header {
                table.add_header_row(header);
            } else {
                table.add_body_row(header);
            }
        }
        Err(_) => (),
    }
    let records = csv_reader.records();
    for r in records {
        match r {
            Ok(record) => table.add_body_row(record.iter().collect::<Vec<&str>>()),
            Err(_) => (),
        }
    }
    table.to_html_string()
}

fn main() {
    let args = Args::parse();
    let reader = csv::Reader::from_path(args.filename);

    match reader {
        Ok(mut r) => {
            let html: String = csv_to_html(&mut r, &!args.no_header);
            match args.output {
                Some(path) => fs::write(path, html).expect("Unable to write file"),
                None => println!("{}", html),
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
