//! Count files in a directory tree, with flexible grouping options.
//! Behavior matches `count_files.py` in the parent tools directory.

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "count-files",
    about = "Count files in a directory tree, grouped by subfolder depth or extension."
)]
struct Args {
    /// Root directory to scan.
    directory: PathBuf,

    /// Grouping depth (>= 0). 0 = single total, 1 = immediate subfolders, …
    #[arg(short = 'd', long = "depth", default_value_t = 1)]
    depth: i32,

    /// Comma-separated extensions to include (e.g. 'png,jpg'). Case-insensitive.
    #[arg(long = "include-ext")]
    include_ext: Option<String>,

    /// Comma-separated extensions to exclude. Applied after --include-ext.
    #[arg(long = "exclude-ext")]
    exclude_ext: Option<String>,

    /// Group results by subfolder at the given --depth (default when no grouping flag is set).
    #[arg(long = "group-by-folder")]
    group_by_folder: bool,

    /// Group results by file extension. Combine with --group-by-folder for "folder | .ext".
    #[arg(long = "group-by-ext")]
    group_by_ext: bool,
}

fn parse_extensions(raw: Option<&str>) -> std::collections::HashSet<String> {
    let Some(s) = raw.filter(|x| !x.is_empty()) else {
        return std::collections::HashSet::new();
    };
    s.split(',')
        .map(|p| p.trim().trim_start_matches('.').to_lowercase())
        .filter(|p| !p.is_empty())
        .collect()
}

fn bucket_key(rel: &Path, depth: usize) -> String {
    let parent = rel.parent().unwrap_or_else(|| Path::new(""));
    let dir_parts: Vec<&str> = parent
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .collect();

    if depth == 0 || dir_parts.is_empty() {
        return "(root)".to_string();
    }

    let n = dir_parts.len().min(depth);
    dir_parts[..n].join(std::path::MAIN_SEPARATOR_STR)
}

fn extension_key(path: &Path) -> String {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    if ext.is_empty() {
        "(no ext)".to_string()
    } else {
        format!(".{ext}")
    }
}

fn raw_extension(path: &Path) -> String {
    path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn count_files(
    directory: &Path,
    depth: usize,
    include_ext: &std::collections::HashSet<String>,
    exclude_ext: &std::collections::HashSet<String>,
    group_by_folder: bool,
    group_by_ext: bool,
) -> HashMap<String, u64> {
    let effective_folder = group_by_folder || !group_by_ext;
    let mut counts: HashMap<String, u64> = HashMap::new();

    for entry in WalkDir::new(directory)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path_is_symlink() {
            continue;
        }

        let full = entry.path();
        let ext = raw_extension(full);
        if !include_ext.is_empty() && !include_ext.contains(&ext) {
            continue;
        }
        if !exclude_ext.is_empty() && exclude_ext.contains(&ext) {
            continue;
        }

        let rel = match full.strip_prefix(directory) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let ext_key = extension_key(full);
        let folder_key = bucket_key(rel, depth);

        let key = if effective_folder && group_by_ext {
            format!("{folder_key} | {ext_key}")
        } else if group_by_ext {
            ext_key
        } else {
            folder_key
        };

        *counts.entry(key).or_insert(0) += 1;
    }

    counts
}

fn main() {
    let args = Args::parse();

    let directory = match fs::canonicalize(&args.directory) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: directory not found: {} ({e})", args.directory.display());
            std::process::exit(1);
        }
    };

    if !directory.is_dir() {
        eprintln!("error: not a directory: {}", directory.display());
        std::process::exit(1);
    }

    if args.depth < 0 {
        eprintln!("error: --depth must be >= 0");
        std::process::exit(2);
    }

    let depth = args.depth as usize;
    let include_ext = parse_extensions(args.include_ext.as_deref());
    let exclude_ext = parse_extensions(args.exclude_ext.as_deref());

    let counts = count_files(
        &directory,
        depth,
        &include_ext,
        &exclude_ext,
        args.group_by_folder,
        args.group_by_ext,
    );

    if counts.is_empty() {
        println!("No files found.");
        return;
    }

    let mut keys: Vec<_> = counts.keys().cloned().collect();
    keys.sort();

    let col_width = keys.iter().map(|k| k.len()).max().unwrap_or(0) + 2;
    let mut total: u64 = 0;

    for key in &keys {
        let n = counts[key];
        total += n;
        println!("{key:<col_width$}{n:>8}");
    }

    println!("{}", "-".repeat(col_width + 8));
    println!("{:<col_width$}{total:>8}", "TOTAL");
}
