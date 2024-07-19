use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    str::FromStr,
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

    let mut releases: BTreeMap<MajorMinor, Vec<Release>> = BTreeMap::new();

    for window in dividing_line_indices.windows(2) {
        let current_dividing_line = window[0];
        let next_dividing_line = window[1];

        let start_index = current_dividing_line - 1;
        let end_index = next_dividing_line - 1;

        let release = rust_version(&destination, &lines[start_index..end_index]);
        releases
            .entry(release.major_minor)
            .and_modify(|list| list.push(release.clone()))
            .or_insert(vec![release]);
    }

    // Last version is a special case
    let last_start_index = dividing_line_indices[dividing_line_indices.len() - 1] - 1;
    let release = rust_version(&destination, &lines[last_start_index..]);
    releases
        .entry(release.major_minor)
        .and_modify(|list| list.push(release.clone()))
        .or_insert_with(|| vec![release]);

    let summary: Vec<_> = releases
        .iter()
        .rev()
        .map(|(key, values)| {
            if values.len() == 1 {
                format!(
                    "- [{key}.{} ({})]({key}.md)",
                    values[0].patch, values[0].date
                )
            } else {
                let patches: Vec<_> = values.iter().map(|v| v.patch.to_string()).collect();
                format!(
                    "- [{key}.{{{}}} ({})]({key}.md)",
                    patches.join(", "),
                    values.last().unwrap().date
                )
            }
        })
        .collect();

    fs::write(destination.join("SUMMARY.md"), summary.join("\n"))
        .expect("writing summary should work");
}

fn rust_version<P: AsRef<Path>>(destination: P, markdown: &[String]) -> Release {
    let rust_version_headline = &markdown[0];

    let release: Release = rust_version_headline
        .parse()
        .expect("should have been able to parse a Release");

    let filename = format!("{}.md", release.major_minor);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination.as_ref().join(&filename))
        .expect("should be able to create/open a version's file");

    file.write_all(markdown.join("\n").as_bytes())
        .expect("writing a version's file should work");
    file.write_all("\n".as_bytes())
        .expect("writing should work");

    release
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct MajorMinor {
    major: u32,
    minor: u32,
}

impl fmt::Display for MajorMinor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug, Clone)]
struct Release {
    major_minor: MajorMinor,
    patch: u32,
    pre: Option<String>,
    date: String,
}

impl FromStr for Release {
    type Err = String;

    fn from_str(rust_version_headline: &str) -> Result<Self, Self::Err> {
        let mut parts = rust_version_headline.split(" ");
        parts.next().unwrap(); // "Version"

        let number = parts
            .next()
            .expect("should have had a number after 'version '");
        let mut number_parts = number.split(".");
        let major: u32 = number_parts
            .next()
            .ok_or_else(|| String::from("should have had a major version"))?
            .parse()
            .map_err(|e| format!("major version should have been a number: {e}"))?;
        let minor: u32 = number_parts
            .next()
            .ok_or_else(|| String::from("should have had a minor version"))?
            .parse()
            .map_err(|e| format!("minor version should have been a number: {e}"))?;

        let (patch, pre) = match number_parts.next() {
            Some(s) => {
                let mut patch_pre_parts = s.split("-");
                let patch = patch_pre_parts
                    .next()
                    .map(|p| {
                        p.parse().map_err(|e| {
                            format!("patch version should have been a number: {e}: {p}")
                        })
                    })
                    .transpose()?
                    .unwrap_or(0);
                let pre = patch_pre_parts.next().map(ToString::to_string);

                (patch, pre)
            }
            None => (0, None),
        };

        let date = parts
            .next()
            .ok_or_else(|| String::from("should have had a date after the number"))?
            .replace("(", "")
            .replace(")", "");

        Ok(Self {
            major_minor: MajorMinor { major, minor },
            patch,
            pre,
            date,
        })
    }
}
