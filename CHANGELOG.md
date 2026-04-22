# Changelog

All notable changes to this project will be documented in this file.

## ctor [0.10.1] - 2026-04-22
## dtor [0.8.1] - 2026-04-22
## link-section [0.2.1] - 2026-04-22

### Added

- Included licenses in all files.
- Bumped proc-macro dependency versions.
- `dtor` crate exports `native` module with `at_binary_exit` and `at_library_exit` functions.

### Fixed

- Fix MSRV in ctor docs.
- Various hardening fixes under Miri.
- Adding priority to `ctor`s accidentally enabled the anonymous flag.

### Changed

- `link-section` crate no longer offers `const` section pointers.
- `ctor` exports all `dtor` macros from `dtor` crate rather than reimplementing them.
