use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs::OpenOptions, io::Write};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct Problem {
    slug: String,
    rust: bool,
    golang: bool,
    python: bool,
}

impl Problem {
    fn entry_filter(entry: &DirEntry) -> bool {
        entry
            .path()
            .to_str()
            .map(|s| !(s.starts_with("/.") || s.contains("/target/") || s.contains(".mypy_cache")))
            .unwrap_or(true)
    }

    fn collect() -> HashMap<String, Problem> {
        let re = Regex::new(r".*/(\d{4}-[^/]*).*").unwrap();
        let mut problems: HashMap<String, Problem> = HashMap::new();

        let walker = WalkDir::new("..").into_iter();
        for entry in walker.filter_entry(Self::entry_filter) {
            let path = entry.unwrap();
            let path = path.path().to_str().unwrap();

            let Some((_, [slug])) = re.captures(path).map(|caps| caps.extract()) else {
                continue;
            };

            problems
                .entry(slug.to_string())
                .and_modify(|e| {
                    if path.contains("main.rs") || path.contains("Cargo.toml") {
                        e.rust = true;
                    }
                    if path.contains("main.go") || path.contains("go.mod") {
                        e.golang = true;
                    }
                    if path.contains(".py") {
                        e.python = true;
                    }
                })
                .or_insert(Problem {
                    slug: slug.to_string(),
                    rust: false,
                    golang: false,
                    python: false,
                });
        }

        problems
    }

    fn sort(problems: HashMap<String, Problem>) -> Vec<Problem> {
        let mut problems: Vec<_> = problems.into_iter().map(|x| x.1).collect();
        problems.sort_by(|a, b| a.slug.cmp(&b.slug));
        problems
    }
}

fn main() -> Result<()> {
    let problems = Problem::sort(Problem::collect());

    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open("../README.md")?;

    file.write_all("# Index / TOC\n\n".as_bytes())?;
    file.write_all("| Problem | Rust | Golang | Python |\n".as_bytes())?;
    file.write_all("|:--------|:----:|:------:|:------:|\n".as_bytes())?;

    let (mut count, mut count_rust, mut count_golang, mut count_python) = (0, 0, 0, 0);
    for problem in problems.iter() {
        count += 1;
        if problem.rust {
            count_rust += 1;
        }
        if problem.golang {
            count_golang += 1;
        }
        if problem.python {
            count_python += 1;
        }
    }
    file.write_fmt(format_args!(
        "| {} | {} | {} | {} |\n",
        count, count_rust, count_golang, count_python
    ))?;

    for problem in problems {
        file.write_fmt(format_args!(
            "| {} | {} | {} | {} |\n",
            problem.slug,
            if problem.rust { '✅' } else { ' ' },
            if problem.golang { '✅' } else { ' ' },
            if problem.python { '✅' } else { ' ' },
        ))?;
    }

    file.write_all("\n\n# License\n\n[WTFPL](./LICENSE.md)\n".as_bytes())?;
    Ok(())
}
