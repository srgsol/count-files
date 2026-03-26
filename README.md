# count-files

Rust CLI to count files under a directory, grouped by subfolder depth and/or extension.

Regular files are walked recursively; symbolic links are skipped. Extensions in filters are compared case-insensitively (leading dots optional in `--include-ext` / `--exclude-ext`).

## Installation

- **Binary:** [Releases](https://github.com/srgsol/count-files/releases) — download the archive for your OS, unpack, run `./count-files` (or add the binary to your `PATH`).
- **Cargo:** with [Rust](https://www.rust-lang.org/tools/install) installed:

```bash
cargo install --locked --git https://github.com/srgsol/count-files.git
```

## Usage

```text
count-files [OPTIONS] <DIRECTORY>
```

Run `count-files --help` for the full option list.

| Option | Description |
|--------|-------------|
| `-d`, `--depth` | Grouping depth (≥ 0). `0` = one total; `1` = immediate subfolders; higher values use deeper path prefixes. Default: `1`. |
| `--include-ext` | Comma-separated extensions to keep (e.g. `png,jpg`). |
| `--exclude-ext` | Comma-separated extensions to drop; applied after `--include-ext`. |
| `--group-by-folder` | Bucket by folder at the chosen `--depth` (this is the default when no grouping flag is set). |
| `--group-by-ext` | Bucket by extension. With `--group-by-folder`, printed keys combine folder path and extension with a pipe between them (as in the program output). |

### Examples

```bash
# Count per top-level subfolder (default depth 1)
count-files .

# Single total for the whole tree
count-files -d 0 .

# Count by extension only
count-files --group-by-ext .

# Per-folder and per-extension
count-files --group-by-folder --group-by-ext .

# Only certain image types
count-files --include-ext png,jpg,webp ./assets
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
