use clap::Parser;
use std::fs;

/// Convert CSV to HTML table
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
    #[arg(short, long, default_value_t = String::from(","))]
    delimiter: String,
}

fn main() {
    let args = Args::parse();
    let csv_file = fs::read_to_string(&args.filename).expect("Cannot read file");
    let delim = args.delimiter.as_bytes()[0];
    let html: String = csv_to_html::convert(&csv_file, &delim, &!args.no_header);
    match args.output {
        Some(path) => fs::write(path, html).expect("Unable to write file"),
        None => println!("{}", html),
    }
}
