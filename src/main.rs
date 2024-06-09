use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("RELEASES.md").expect("file read"));

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().expect("should have valid lines");

    let dividing_line_indices: Vec<_> = lines.iter().enumerate().filter_map(|(index, line)| {
        (!line.is_empty() && line.chars().all(|c| c == '=')).then_some(index)
    }).collect();

    for window in dividing_line_indices.windows(2) {
        let current_dividing_line = window[0];
        let next_dividing_line = window[1];

        let start_index = current_dividing_line - 1;
        let end_index = next_dividing_line - 1;

        for line in &lines[start_index..end_index] {
            println!("{line}");
        }
    }
}
