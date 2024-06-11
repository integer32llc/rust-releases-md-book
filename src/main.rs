use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let destination = Path::new("target/book/src");
    fs::create_dir_all(&destination).expect("create dir should work");

    let reader = BufReader::new(File::open("RELEASES.md").expect("file read"));

    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("should have valid lines");

    let dividing_line_indices: Vec<_> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            (!line.is_empty() && line.chars().all(|c| c == '=')).then_some(index)
        })
        .collect();

    let mut summary = Vec::with_capacity(dividing_line_indices.len());

    for window in dividing_line_indices.windows(2) {
        let current_dividing_line = window[0];
        let next_dividing_line = window[1];

        let start_index = current_dividing_line - 1;
        let end_index = next_dividing_line - 1;

        summary.push(rust_version(&destination, &lines[start_index..end_index]));
    }

    let last_start_index = dividing_line_indices[dividing_line_indices.len() - 1] - 1;
    summary.push(rust_version(&destination, &lines[last_start_index..]));

    fs::write(destination.join("SUMMARY.md"), summary.join("\n"))
        .expect("writing summary should work");
}

fn rust_version<P: AsRef<Path>>(destination: P, markdown: &[String]) -> String {
    let rust_version_headline = &markdown[0];
    let rust_version_filename = format!(
        "{}.md",
        rust_version_headline
            .replace(" ", "_")
            .replace("(", "")
            .replace(")", "")
            .to_lowercase()
    );

    fs::write(
        destination.as_ref().join(&rust_version_filename),
        markdown.join("\n"),
    )
    .expect("writing a version's file should work");

    format!("- [{rust_version_headline}]({rust_version_filename})")
}
