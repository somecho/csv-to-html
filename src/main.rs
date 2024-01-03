use clap::Parser;
use std::fs;

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

fn main() {
    let args = Args::parse();
    let csv_file = fs::read_to_string(&args.filename).expect("Cannot read file");
    let html: String = tbl::csv_to_html(&csv_file, &!args.no_header);
    match args.output {
        Some(path) => fs::write(path, html).expect("Unable to write file"),
        None => println!("{}", html),
    }
}
