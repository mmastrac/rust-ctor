# Changelog

All notable changes to this project will be documented in this file.

## ctor [0.11.0] - Unreleased
## dtor [0.11.0] - Unreleased
## link-section [0.11.0] - Unreleased

### Added

- Added `method` attribute to `dtor` macro.
- Added `link_section` attribute to `dtor` macro.
- AIX support for `ctor`/`dtor` crates.

### Changed

- Significant rewrite to ctor/dtor macros and documentation.
- Renamed `at_library_exit` to `at_module_exit` in `dtor` crate.
- Macro attributes and crate features are auto-documented.
- Rewrote `statics` code in `ctor` to not require `std`.

### Removed

- `cxa_atexit` feature from `dtor` crate. (appropriate method is now used per-platform)
- `export_native` feature from `dtor` crate. (natives always exported)

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
