# `bump-version`
Read, bump, write, commit and tag versions, automatically, in one line!

## Installation
Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Using Cargo (recommended)
```bash
> cargo install bump-version
```

### Building from source
```bash
> git clone https://github.com/ssiyad/bump-version
> cd bump-version
> cargo build --release
> cp target/release/bump-version /usr/local/bin
```

## Example
```
> bump-version cargo.toml --dry-run
File: Some("cargo.toml")
Bump type: patch
Dry run: true
Commit: false
Tag: false

> bump-version cargo.toml --bump-type minor
File: Some("cargo.toml")
Bump type: minor
Dry run: false
Commit: false
Tag: false

> bump-version package.json --bump-type major --commit --tag
File: Some("package.json")
Bump type: major
Dry run: false
Commit: true
Tag: true
```

## Help
```bash
> bump-version --help
```

```
Bump the version in package.json or Cargo.toml files

Usage: bump-version [OPTIONS] [FILE]

Arguments:
  [FILE]  File path to the package.json or cargo.toml file

Options:
      --bump-type <BUMP_TYPE>  [default: patch]
      --dry-run                
      --commit                 Create a commit after bumping version
      --tag                    Create a tag after bumping version
  -h, --help                   Print help
```

## Self Promotion
Like this project? Give it a star! ⭐, and spread the word! 🚀. And if you are
feeling especially charitable, follow [Sabu Siyad](https://ssiyad.com) on
[GitHub](https://github.com/ssiyad). Thanks!
