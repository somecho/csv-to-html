use build_html::{Html, Table};

pub fn convert(csv: &String, has_header: &bool) -> String {
    let mut csv_reader = csv::Reader::from_reader(csv.as_bytes());
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
