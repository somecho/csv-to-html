use build_html::{Html, Table};
use clap::Parser;

/// Convert CSV table to HTML table
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// path to input file
    #[arg(short, long)]
    filename: String,
    /// the CSV file has no header
    #[arg(short, long)]
    no_header: bool,
}

fn csv_to_html(csv_reader: &mut csv::Reader<std::fs::File>, has_header: &bool) -> String {
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
        Err(_) => {}
    }
    let records = csv_reader.records();
    for r in records {
        match r {
            Ok(record) => {
                table.add_body_row(record.iter().collect::<Vec<&str>>());
            }
            Err(_) => {}
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
            println!("{}", html);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
