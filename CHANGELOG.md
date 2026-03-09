# Changelog

All notable changes to vecli will be documented here.

## [0.2.0] - 09/03/2026

### Added
- Global flags via `Flag::global()` — available to all commands automatically
- App-level main entry point via `.main()` for REPL-style usage
- `PassedFlags` type passed to the main entry handler
- Flags now listed in command-level help output
- App-level `strict_flags` support
- `GLOBAL FLAGS` and `OPTIONS` sections in app-level help
- Aligned column widths in help output across all sections
- `Flag::global()` constructor
- `format_flag` utility for consistent flag formatting

### Fixed
- Global flags now correctly merge into `CommandContext.flags` for handlers
- Strict flag mode on commands no longer errors on valid global flags
- Help column misalignment between COMMANDS and OPTIONS sections
- `--help` now correctly resolves command-specific help regardless of flag position

### Changed
- `show_help_if_no_args` and `show_help_on_fail` renamed to `print_help_if_no_args` and `print_help_on_fail`
- `Command`, `Flag`, and `CommandContext` split into dedicated modules (`commands.rs`, `flags.rs`)

---

## [0.1.3] - 09/03/2026

### Fixed
- Minor spelling and wording corrections in documentation

---

## [0.1.2] - 09/03/2026

### Fixed
- Minor bug fixes

---

## [0.1.1] - 09/03/2026

### Fixed
- Minor bug fixes

---

## [0.1.0] - 09/03/2026

### Added
- Initial release
- `App` builder with `.name()`, `.description()`, `.version()`, `.add_command()`
- `Command` builder with `.description()`, `.usage()`, `.flag()`, `.strict_flags()`
- `Flag` builder with `.alias()`, `.description()`
- `CommandContext` with `subcommand`, `positionals`, and `flags`
- Built-in `--help` and `--version` handling
- Per-command help with usage strings
- Alias resolution for short flags
- `Terminal` with `prompt`, `confirm`, and `choice`
- `Confirm` and `Choice` builder types
- Zero dependencies
