//! Core application builder and dispatch logic.
//!
//! The [`App`] struct is the entry point for every vecli program. Configure it
//! with the builder methods, register commands and flags, then call [`App::run`]
//! to hand control to the framework.

use crate::utils::{dispatch, format_flag, parse_flags};
use crate::*;

/// The top-level CLI application builder.
///
/// Construct with [`App::new`], chain configuration methods, register commands
/// with [`App::add_command`], then call [`App::run`] to parse `std::env::args`
/// and dispatch to the appropriate handler.
///
/// # Example
/// ```
/// use vecli::{App, Command, CommandContext};
///
/// fn greet(_ctx: &CommandContext) {
///     println!("Hello!");
/// }
///
/// let app = App::new("mytool")
///     .name("My Tool")
///     .version("1.0.0")
///     .add_command(Command::new("greet", greet));
///
/// // app.run();
/// ```
#[derive(Default)]
#[must_use = "App does nothing until you call `.run()`"]
pub struct App {
    pub(crate) prog: String,
    pub(crate) name: String,
    description: String,
    version: String,
    main_entrypoint: Option<fn(PassedFlags)>,
    commands: Vec<Command>,
    flags: Vec<Flag>,
    print_help_if_no_args: bool,
    print_help_on_fail: bool,
    strict_flags: bool,
}

impl App {
    /// Creates a new `App` with the given program name.
    ///
    /// `prog_name` is used in usage strings and error messages (typically the
    /// binary name, e.g. `"mytool"`).
    pub fn new(prog_name: impl Into<String>) -> Self {
        Self {
            prog: prog_name.into(),
            ..Default::default()
        }
    }

    /// Sets the display name shown in help and version output.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the description shown in the app-level help output.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Sets the version string shown by `--version`.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// When `true`, prints help and exits if no arguments are provided.
    ///
    /// Mutually exclusive with [`App::main`]. If both are set, the main entry
    /// point takes priority and a warning is printed to stderr.
    pub fn print_help_if_no_args(mut self, show: bool) -> Self {
        self.print_help_if_no_args = show;
        self
    }

    /// When `true`, prints the full help listing after any dispatch error.
    pub fn print_help_on_fail(mut self, show: bool) -> Self {
        self.print_help_on_fail = show;
        self
    }

    /// When `true`, aborts with an error if an unknown app-level flag is passed.
    ///
    /// When `false` (the default), unknown flags produce a warning and execution continues.
    /// Per-command strict mode is configured separately via [`Command::strict_flags`].
    pub fn strict_flags(mut self, strict: bool) -> Self {
        self.strict_flags = strict;
        self
    }

    /// Returns the registered command with the given name, if any.
    fn _find_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }

    /// Registers a command built with the [`Command`] builder.
    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Registers a flag on the app.
    ///
    /// If the flag was created with [`Flag::global`], it is available to all commands
    /// and merged into [`CommandContext::flags`] automatically. Otherwise it is treated
    /// as an entry-point flag, visible in the OPTIONS section of help and delivered
    /// via [`PassedFlags`] to the main entry handler.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Registers a command from individual parameters without the [`Command`] builder.
    ///
    /// Prefer [`App::add_command`] for most cases. This variant is useful when constructing
    /// commands dynamically at runtime.
    pub fn add_command_param(
        mut self,
        name: impl Into<String>,
        flags: Option<Vec<Flag>>,
        description: impl Into<String>,
        handler: fn(&CommandContext),
        usage: Option<impl Into<String>>,
        strict_flags: bool,
        print_help_if_no_args: bool,
        subcommands: Vec<Command>,
    ) -> Self {
        self.commands.push(Command {
            name: name.into(),
            description: description.into(),
            handler: Some(handler),
            usage: usage.map(|u| u.into()),
            known_flags: flags.unwrap_or_default(),
            strict_flags,
            print_help_if_no_args,
            subcommands,
        });
        self
    }

    /// Registers a handler called when no subcommand is provided.
    ///
    /// The handler receives a [`PassedFlags`] map containing any flags the user
    /// passed before a subcommand. Mutually exclusive with [`App::print_help_if_no_args`];
    /// if both are set, this handler takes priority.
    pub fn main(mut self, entry: fn(PassedFlags)) -> Self {
        self.main_entrypoint = Some(entry);
        self
    }

    /// Returns `fall_to` if `field` is empty, otherwise returns `field`.
    fn _get_else(&self, field: &str, fall_to: &str) -> String {
        if field.is_empty() {
            fall_to.to_string()
        } else {
            field.to_string()
        }
    }

    /// Partitions registered flags into `(global_flags, entry_flags)`.
    ///
    /// Global flags are made available to all commands. Entry flags are delivered
    /// only to the main entry handler via [`PassedFlags`].
    fn _get_flags(&self) -> (Vec<Flag>, Vec<Flag>) {
        self.flags.iter().cloned().partition(|flag| flag.is_global)
    }

    /// Prints the app-level help text to stdout.
    ///
    /// Output includes the app name, version, description, COMMANDS listing,
    /// OPTIONS (entry-point flags and `--version`), and GLOBAL FLAGS.
    pub fn print_help(&self) {
        if !self.name.is_empty() {
            println!(
                "{} v{}",
                self.name,
                self._get_else(&self.version, "<unknown>")
            );
        }
        println!("USAGE: {} <command> [options]", self.prog);
        if !self.description.is_empty() {
            println!();
            println!("    {}", self.description);
        }
        println!();

        let longest = self
            .commands
            .iter()
            .map(|c| c.name.len())
            .chain(["help, -h", "version"].iter().map(|s| s.len()))
            .max()
            .unwrap_or(0)
            + 10;

        if !self.commands.is_empty() {
            println!("COMMANDS:");
            for cmd in &self.commands {
                println!(
                    "    {:<width$} {}",
                    cmd.name,
                    cmd.description,
                    width = longest
                );
            }
            println!();
        } else {
            println!("No commands available. Add some using .add_command()!");
            println!();
        }

        let (global_flags, entry_flags) = self._get_flags();

        println!("OPTIONS:");
        println!(
            "    {:<width$} print the app name and version and exit",
            "--version",
            width = longest
        );
        for flag in entry_flags {
            let left = format_flag(&flag.name, flag.alias.as_deref());
            let description = flag.description.as_deref().unwrap_or("");
            println!("    {:<width$} {}", left, description, width = longest);
        }
        println!();

        println!("GLOBAL FLAGS:");
        println!(
            "    {:<width$} print this help message and exit",
            "--help, -h",
            width = longest
        );
        for flag in global_flags {
            let left = format_flag(&flag.name, flag.alias.as_deref());
            let description = flag.description.as_deref().unwrap_or("");
            println!("    {:<width$} {}", left, description, width = longest);
        }
    }

    /// Parses `std::env::args`, resolves aliases, and dispatches to the matching command handler.
    ///
    /// Handles the following built-in flags before reaching any user-defined handler:
    /// - `--help` / `-h`: prints command-specific or app-level help and exits.
    /// - `--version`: prints the app name and version and exits.
    ///
    /// If no subcommand is provided and a main entry point is registered, the entry
    /// handler is called with the resolved flags. If no subcommand is found in the
    /// registry, an error is printed and the function returns without calling any handler.
    /// When `print_help_on_fail` is set, the full help listing is also printed.
    pub fn run(self) {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let parsed_flags = parse_flags(&args);

        if args.is_empty() {
            // feature thought
            // - use custom exit code system? unnecessary but fun. programmer returns 0, default entry returns 404
            //   CHANGES? change self.main_entrypoint type to Option<fn() -> i32> and do accordingly

            if let Some(_) = self.main_entrypoint
                && self.print_help_if_no_args
            {
                eprintln!(
                    "warning: App entry point and field print_help_if_no_args are mutually exclusive. App entry point takes priority."
                );
            }

            if let Some(main_entrypoint) = self.main_entrypoint {
                main_entrypoint(PassedFlags {
                    map: parse_flags(&args),
                });
                return;
            }

            if self.print_help_if_no_args {
                self.print_help();
                return;
            }
        }

        let subcommand_name = args.iter().find(|a| !a.starts_with('-'));

        if parsed_flags.contains_key("help") || parsed_flags.contains_key("h") {
            if let Some(name) = subcommand_name
                && let Some(command) = self._find_command(name)
            {
                command.print_help(&self.prog);
                return;
            }
            self.print_help();
            return;
        }

        if parsed_flags.contains_key("version") {
            println!(
                "{} v{}",
                self.name,
                self._get_else(&self.version, "<unknown>")
            );
            return;
        }

        let (global_flags, entry_flags) = self._get_flags();
        let all_app_flags: Vec<&Flag> = global_flags.iter().chain(entry_flags.iter()).collect();

        let mut canonical_flags = std::collections::HashMap::new();
        for (key, value) in &parsed_flags {
            let canonical = all_app_flags
                .iter()
                .find(|f| f.alias.as_deref() == Some(key.as_str()))
                .map(|f| f.name.clone())
                .unwrap_or_else(|| key.clone());
            canonical_flags.insert(canonical, value.clone());
        }

        for parsed_flag in canonical_flags.keys() {
            if matches!(parsed_flag.as_str(), "help" | "h" | "version") {
                continue;
            }
            let is_known = all_app_flags.iter().any(|f| f.name == *parsed_flag);
            if !is_known && self.commands.is_empty() {
                if self.strict_flags {
                    println!("error: Unknown flag '--{}'.", parsed_flag);
                    return;
                }
                println!("warning: Unknown flag '--{}'.", parsed_flag);
            }
        }

        let Some(subcommand) = args.iter().find(|a| !a.starts_with('-')) else {
            if let Some(main_entrypoint) = self.main_entrypoint {
                main_entrypoint(PassedFlags {
                    map: canonical_flags,
                });
            } else {
                println!("error: No command provided. Try '{} --help'.", self.prog);
            }
            return;
        };

        let subcommand = subcommand.to_owned();
        let Some(command) = self._find_command(&subcommand) else {
            println!(
                "error: No such command '{}', try '{} --help' for help.",
                subcommand, self.prog
            );
            if self.print_help_on_fail {
                self.print_help();
            }
            return;
        };

        let (global_flags, _) = self._get_flags();
        dispatch(
            command,
            &args[1..],
            canonical_flags,
            &global_flags,
            &self.prog,
        );
    }
}

// FUTURE FEATURES? //
// after 0.1.0 (base):
// - commands use their docstring as the description?
