# `bump-version`
Read, bump, write, commit and tag versions, automatically, in one line!

## Example
```
» bump-version --cargo-toml --dry-run
[DEBUG] Parsed source: Cargo.toml
[INFO ] Bumped version from 0.26.1 to 0.26.2

» bump-version --cargo-toml --dry-run minor
[DEBUG] Parsed source: Cargo.toml
[INFO ] Bumped version from 0.26.1 to 0.27.0

» bump-version --cargo-toml --dry-run major
[DEBUG] Parsed source: Cargo.toml
[INFO ] Bumped version from 0.26.1 to 1.0.0
```

## Help
```bash
bump-version --help
```

```
Usage: bump-version [OPTIONS] [bump-type]

Arguments:
  [bump-type]  Bump type [default: patch] [possible values: major, minor, patch]

Options:
      --package-json  Update package.json
      --cargo-toml    Update Cargo.toml
      --dry-run       Do not write to sources
      --no-commit     Do not commit the version change
      --no-tag        Do not add a tag
  -h, --help          Print help
  -V, --version       Print version
```

## Self Promotion
Like this project? Give it a star! ⭐, and spread the word! 🚀. And if you are
feeling especially charitable, follow [ssiyad](https://ssiyad.com) on
[GitHub](https://github.com/ssiyad)
