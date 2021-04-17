use std::fs;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::{self, Context};
use clap::Clap;
// use glob::glob;

/// numberfile: A simple application for numbering files to order them
#[derive(Clap)]
#[clap(version = "0.1", author = "drmason13")]
struct Args {
    /// The path to the files to number.
    /// Using path to a folder will number all files within the folder.
    /// TODO: Globs can be used to match certain files within a folder.
    #[clap(default_value = "./")]
    path: String,

    /// The delimiter between number prefix and the file name
    #[clap(default_value = "-", short)]
    delimiter: String,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    // TODO: Globs can be used to match certain files within a folder.
    let mut files_to_number = fs::read_dir(&args.path)?
        .filter_map(|entry| {
            entry.ok().and_then(
                |entry| entry.file_type().ok().and_then(|file_type| {
                    if file_type.is_file() {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
            )
        })
        .collect::<Vec<_>>();

    let number_of_digits = std::cmp::max(2, (files_to_number.len() as f32).log10().round() as usize + 1);

    // TODO: Sort by Created time instead of the default: Modified time
    // TODO: Sort by File Name instead of the default: Modified time
    files_to_number.sort_by_key(|path| {
        path.metadata()
            .with_context(|| format!("Could not access metadata for file {}", path.display())).unwrap()
            .modified()
            .with_context(|| {
                format!(
                    "Accssing file modified time not supported on this platform {}",
                    path.display()
                )
            }).unwrap()
    });

    for (n, path) in files_to_number.iter().enumerate() {
        let name = Path::new(path.file_name().unwrap_or(OsStr::new("")));

        // n + 1 because we want 1 indexed numbers
        let numbered_name: PathBuf = Path::new(&format!(
            "{:03$}{}{}",
            n + 1, args.delimiter, name.display(), number_of_digits
        )).into();

        let from = path;
        let to = if let Some(parent) = path.parent() {
            parent.join(numbered_name)
        } else {
            numbered_name
        };

        fs::rename(&from, &to).context(format!("rename {} to {}", &from.display(), &to.display()))?;
    }

    Ok(())
}
