pub fn print_markdown_table(data: &Vec<Vec<String>>) {
    let max_lengths = calc_max_column_length(data);

    for (i, row) in data.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            let width = max_lengths[i];
            print!("| {:>width$} ", cell);
        }
        print!("|");
        println!();

        if i == 0 {
            println!("{}", generate_header_line(&max_lengths));
        }
    }
}

fn generate_header_line(max_lengths: &[usize]) -> String {
    let mut header_line = "".to_string();
    for max_length in max_lengths.iter() {
        header_line.push_str("| ");
        header_line.push_str("-".repeat(*max_length).as_str());
        header_line.push(' ');
    }
    header_line.push(' ');
    header_line
}

fn calc_max_column_length(data: &Vec<Vec<String>>) -> Vec<usize> {
    let mut max_len: Vec<usize> = vec![0; data[0].len()];

    for row in data {
        for (i, col) in row.iter().enumerate() {
            if max_len[i] < col.len() {
                max_len[i] = col.len();
            }
        }
    }

    max_len
}
