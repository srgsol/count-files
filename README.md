# count-files

Rust CLI to count files under a directory, grouped by subfolder depth and/or extension.

Regular files are walked recursively; symbolic links are skipped. Extensions in filters are compared case-insensitively (leading dots optional in `--include-ext` / `--exclude-ext`).

## Installation

### Prebuilt binaries (GitHub Releases)

Push a version tag (for example `v0.1.0`) to GitHub. The workflow in `.github/workflows/release.yml` builds Linux (x86_64), macOS (Apple Silicon), and Windows (x86_64) archives and attaches them to a new **Releases** entry.

1. Open your repo’s **Releases** page on GitHub.
2. Download the archive for your OS (`*.tar.gz` or `*.zip`).
3. Extract the `count-files` binary (or `count-files.exe` on Windows).
4. Put it on your `PATH`, or run it with a path (for example `./count-files` from the folder that contains the binary).

### Install with Cargo (from Git)

Requires [Rust](https://www.rust-lang.org/tools/install). Replace the URL with your repository:

```bash
cargo install --locked --git https://github.com/YOUR_USER/count-files.git
```

Pin a release:

```bash
cargo install --locked --git https://github.com/YOUR_USER/count-files.git --tag v0.1.0
```

### Install from crates.io (optional)

If you [`cargo publish`](https://doc.rust-lang.org/cargo/reference/publishing.html) this crate, others can run:

```bash
cargo install count-files
```

### Build from a clone

Requires a recent stable Rust toolchain (edition 2021).

```bash
git clone https://github.com/YOUR_USER/count-files.git
cd count-files
cargo build --release
```

The binary is `target/release/count-files` (`count-files.exe` on Windows). Install for your user:

```bash
cargo install --locked --path .
```

That copies the binary into `~/.cargo/bin` if that directory is on your `PATH`.

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
