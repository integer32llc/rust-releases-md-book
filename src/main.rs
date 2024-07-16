use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    let destination = Path::new("target/book/src");
    fs::remove_dir_all(&destination).expect("remove dir should work");
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

        let summary_line = rust_version(&destination, &lines[start_index..end_index]);
        if !summary_line.is_empty() {
            summary.push(summary_line);
        }
    }

    let last_start_index = dividing_line_indices[dividing_line_indices.len() - 1] - 1;
    let summary_line = rust_version(&destination, &lines[last_start_index..]);
    if !summary_line.is_empty() {
        summary.push(summary_line);
    }

    fs::write(destination.join("SUMMARY.md"), summary.join("\n"))
        .expect("writing summary should work");
}

fn rust_version<P: AsRef<Path>>(destination: P, markdown: &[String]) -> String {
    let rust_version_headline = &markdown[0];

    let mut parts = rust_version_headline.split(" ");
    parts.next().unwrap(); // "Version"

    let number = parts.next().expect("should have had a number after 'version '");
    let mut number_parts = dbg!(number).split(".");
    let major = number_parts.next().expect("should have had a major version");
    let minor = number_parts.next().expect("should have had a minor version");
    let patch = number_parts.next().unwrap_or("0");

    // TODO: handle prerelease

    let date = parts.next().expect("should have had a date after the number")
        .replace("(", "")
        .replace(")", "");

    let filename = format!("{major}.{minor}.md");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination.as_ref().join(&filename))
        .expect("should be able to create/open a version's file");

    file.write_all(markdown.join("\n").as_bytes()).expect("writing a version's file should work");
    file.write_all("\n".as_bytes()).expect("writing should work");

    if patch == "0" {
        format!("- [{major}.{minor}]({filename}), {date}")
    } else {
        String::new()
    }
}
