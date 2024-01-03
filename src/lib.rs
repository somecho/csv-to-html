use std::fs::File;
use build_html::{Html,Table};

pub fn csv_to_html(csv_reader: &mut csv::Reader<File>, has_header: &bool) -> String {
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
