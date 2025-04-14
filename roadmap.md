* 0.1
- [x] Get current version from args
- [x] Calculate major, minor, patch
- [x] Bump patch version
- [x] Return new version

* 0.2
- [x] Get which version to bump
- [x] Calculate major, minor, patch
- [x] Bump selected version
- [x] Return new version

* 0.3
- [x] Parse `package.json`
- [x] Get current version from `package.json`
- [x] Calculate major, minor, patch
- [x] Bump selected version
- [x] Return new version

* 0.4
- [x] Write new version to `package.json`
- [x] Get `package.json` from git root if not in current directory

* 0.5
- [x] Commit version changes
- [ ] Create a tag for the new version
- [ ] Push changes to remote (maybe)

* 0.6
- [ ] `--dry-run` option
- [ ] `--no-commit` option
- [ ] `--no-tag` option
- [ ] `--no-push` option (maybe)

* 0.7
- [ ] `cargo.toml` support
- [ ] `pyproject.toml` support

* 0.8
- [ ] Configurable files
- [ ] Configurable versioning scheme
- [ ] Configurable commit message
