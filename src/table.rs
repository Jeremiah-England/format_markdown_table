pub struct Table {
    headers: Vec<String>,
    column_count: usize,
    rows: Vec<Vec<String>>,
}

impl Table {
    fn new(headers: Vec<String>, rows: Option<Vec<Vec<String>>>) -> Self {
        let rows = rows.unwrap_or(Vec::new());
        assert!(headers.len() != 0, "Headers should not be empty.");
        let column_count = headers.len();
        for row in rows.iter() {
            assert!(
                row.len() == headers.len(),
                "The first row should be as long as the header."
            )
        }
        return Table {
            headers,
            column_count,
            rows,
        };
    }

    fn parse_line(line: &str) -> Vec<String> {
        // line.trim_matches("|");
        let mut elements: Vec<String> = line
            .split("|") // Ignore backslash escaping for now.
            .map(|x| x.trim().to_string())
            .collect();
        assert!(
            elements.remove(0) == "",
            "Should not contain any serious characters before the first pipe ('|')."
        );
        assert!(
            elements.pop().unwrap() == "",
            "Should not contain any serious characters after the last pipe ('|')."
        );
        return elements;
    }

    pub fn parse(raw: &String) -> Self {
        let lines: Vec<&str> = raw.trim().split("\n").collect();
        assert!(
            lines.len() >= 1,
            "A table must have at least a header line."
        );
        let headers = Self::parse_line(lines[0]);

        let mut rows: Vec<Vec<String>> = Vec::new();
        if lines.len() > 2 {
            for line in lines[2..].iter() {
                rows.push(Self::parse_line(line));
            }
        }

        return Self::new(headers, Some(rows));
    }

    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut column_widths: Vec<usize> = Vec::new();
        for i in 0..self.column_count {
            let mut max_width = self.headers[i].len();
            for row in self.rows.iter() {
                let cell_length = row[i].len();
                if cell_length > max_width {
                    max_width = cell_length;
                }
            }
            column_widths.push(max_width);
        }
        return column_widths;
    }

    fn format_row(row: &Vec<String>, column_widths: &Vec<usize>, fill: Option<char>) -> String {
        let mut formatted: String = "|".to_string();

        let fill_char = fill.unwrap_or(' ');
        let fill_str = fill_char.to_string();

        for (i, string) in row.iter().enumerate() {
            // TODO: Support other alignments besides just left align.
            formatted.push(fill_char);
            formatted.push_str(string.as_str());
            formatted.push_str(
                fill_str
                    .repeat(column_widths[i] - string.len() + 1)
                    .as_str(),
            );
            formatted.push('|');
        }
        return formatted;
    }

    fn get_formatting_row(&self) -> Vec<String> {
        let raw_row = [""].repeat(self.column_count);
        return raw_row.iter().map(|x| x.to_string()).collect();
    }

    pub fn format(&self) -> String {
        let column_widths = self.calculate_column_widths();
        let mut buffer_row: String = "".to_string();
        buffer_row.push_str(Self::format_row(&self.headers, &column_widths, None).as_str());
        buffer_row.push('\n');

        let formatting_row = self.get_formatting_row();
        buffer_row.push_str(Self::format_row(&formatting_row, &column_widths, Some('-')).as_str());
        buffer_row.push('\n');

        for row in self.rows.iter() {
            buffer_row.push_str(Self::format_row(&row, &column_widths, None).as_str());
            buffer_row.push('\n');
        }

        return buffer_row;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_just_header() {
        let table = Table::parse(&"| header |".to_string());
        assert_eq!(table.headers, ["header"])
    }

    #[test]
    fn test_parse_just_header_and_buffer_row() {
        let table = Table::parse(&"| header1 | header2 |\n|----|----|".to_string());
        assert_eq!(table.headers, ["header1", "header2"]);
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn test_parse_all_three_row_types() {
        let table =
            Table::parse(&"| header1 | header2 |\n|----|----|\n| cell 1 | cell 2|".to_string());
        assert_eq!(table.headers, ["header1", "header2"]);
        assert_eq!(table.rows, [["cell 1", "cell 2"]]);
    }

    #[test]
    fn test_parse_extra_whitespace() {
        let table = Table::parse(
            &"     | header1 | header2 |   \n  |----|----|  \n | cell 1      | cell 2|\n"
                .to_string(),
        );
        assert_eq!(table.headers, ["header1", "header2"]);
        assert_eq!(table.rows, [["cell 1", "cell 2"]]);
    }
}
