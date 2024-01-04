use build_html::{Html, Table};

pub fn convert(csv: &String, delimiter: &u8, has_header: &bool) -> String {
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(*delimiter)
        .from_reader(csv.as_bytes());
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
    csv_reader.records().into_iter().for_each(|r| match r {
        Ok(record) => table.add_body_row(record.iter().collect::<Vec<&str>>()),
        Err(_) => (),
    });
    table.to_html_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(input: &str, output: &str, delimiter: &u8, has_header: &bool) {
        let input = std::fs::read_to_string(["data", input].join("/")).unwrap();
        let expected = std::fs::read_to_string(["data", output].join("/")).unwrap();
        assert!(convert(&input, delimiter, has_header) == expected);
    }

    #[test]
    fn random_100_100_header() {
        compare("random-100x100.csv", "random-100x100.html", &b',', &true);
    }

    #[test]
    fn random_100_100_no_header() {
        compare(
            "random-100x100.csv",
            "random-100x100-no-header.html",
            &b',',
            &false,
        );
    }

    #[test]
    fn space_delimiter() {
        compare("space.csv", "space.html", &b' ', &true);
    }
}
