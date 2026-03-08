//! Core application and command builder types.
//!
//! This module defines [`App`], [`Command`], [`Flag`], and [`CommandContext`],
//! which together form the public API for constructing and running a CLI.

use crate::utils::parse_flags;

/// Holds the parsed context for a command invocation.
///
/// Passed by reference to every command handler. Contains the resolved subcommand
/// name, any positional arguments (non-flag tokens), and the full set of flags
/// after alias resolution.
pub struct CommandContext {
    /// The subcommand name as typed by the user.
    pub subcommand: String,
    /// Positional arguments, in order, with flags filtered out.
    pub positionals: Vec<String>,
    /// Resolved flags, keyed by canonical name. Boolean flags have the value `"true"`.
    pub flags: std::collections::HashMap<String, String>,
}

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
    prog: String,
    name: String,
    description: String,
    version: String,
    commands: Vec<Command>,
    show_help_if_no_args: bool,
    show_help_on_fail: bool,
}

/// A single registered subcommand.
///
/// Build with [`Command::new`] and configure via the builder methods before
/// passing to [`App::add_command`].
pub struct Command {
    name: String,
    description: String,
    known_flags: Vec<Flag>,
    usage: Option<String>,
    handler: fn(&CommandContext),
    strict_flags: bool,
}

/// A flag definition for a [`Command`].
///
/// Flags can carry an optional short alias (e.g. `"h"` for `"help"`) which is
/// resolved to the canonical name before the handler is called.
#[derive(Default)]
pub struct Flag {
    /// The canonical long name, without the `--` prefix.
    pub name: String,
    /// Optional short alias, without the `-` prefix.
    pub alias: Option<String>,
    /// Human-readable description shown in generated help text.
    pub description: Option<String>,
}

impl Flag {
    /// Creates a new flag with the given canonical name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Sets the short alias for this flag (e.g. `"h"` to match `-h`).
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    /// Sets the description shown in help output.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl Command {
    /// Creates a new command with the given name and handler function.
    ///
    /// The `handler` receives a [`CommandContext`] containing the resolved flags
    /// and positional arguments for this invocation.
    pub fn new(name: impl Into<String>, handler: fn(&CommandContext)) -> Self {
        Self {
            name: name.into(),
            handler,
            description: "".into(),
            known_flags: Vec::new(),
            usage: None,
            strict_flags: false,
        }
    }

    /// Sets the short description shown in the app-level help listing.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the usage string shown when the user runs `<cmd> --help`.
    ///
    /// Displayed as: `<prog> <cmd> <usage>`. For example, passing `"<file> [--output <path>]"`
    /// produces `mytool convert <file> [--output <path>]`.
    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Controls whether unknown flags cause a hard error or just a warning.
    ///
    /// When `true`, passing an unrecognized flag prints an error and exits without
    /// calling the handler. When `false` (the default), a warning is printed and
    /// execution continues.
    pub fn strict_flags(mut self, strict: bool) -> Self {
        self.strict_flags = strict;
        self
    }

    /// Registers a flag definition for this command.
    ///
    /// Registered flags participate in alias resolution and appear in help text.
    /// Can be called multiple times to register multiple flags.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.known_flags.push(flag);
        self
    }
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
    pub fn show_help_if_no_args(mut self, show: bool) -> Self {
        self.show_help_if_no_args = show;
        self
    }

    /// When `true`, prints the full help listing after any dispatch error.
    pub fn show_help_on_fail(mut self, show: bool) -> Self {
        self.show_help_on_fail = show;
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
    ) -> Self {
        self.commands.push(Command {
            name: name.into(),
            description: description.into(),
            handler,
            usage: usage.map(|u| u.into()),
            known_flags: flags.unwrap_or_default(),
            strict_flags,
        });
        self
    }

    /// Prints the app-level help text to stdout.
    ///
    /// Lists the app name, version, description, and all registered commands.
    pub fn print_help(&self) {
        println!("{} v{}", self.name, self.version);
        println!("{}", self.description);
        println!();
        println!("USAGE:");
        println!("  {} <command> [options]", self.prog);
        println!();
        println!("COMMANDS:");
        for cmd in &self.commands {
            println!("  {:<15} {}", cmd.name, cmd.description);
        }
    }

    /// Parses `std::env::args`, resolves aliases, and dispatches to the matching command handler.
    ///
    /// Handles the following built-in flags before reaching any user-defined handler:
    /// - `--help` / `-h`: prints command-specific or app-level help and exits.
    /// - `--version`: prints the app name and version and exits.
    ///
    /// If no subcommand is found, or if an unknown subcommand is given, an error message
    /// is printed and the function returns without calling any handler. When
    /// `show_help_on_fail` is set, the full help listing is also printed.
    pub fn run(self) {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let parsed_flags = parse_flags(&args);
        let mut flags = std::collections::HashMap::new();

        if args.is_empty() && self.show_help_if_no_args {
            self.print_help();
            return;
        }

        let Some(subcommand) = args.first() else {
            println!("error: No command provided. Try '{} --help'.", self.prog);
            return;
        };
        let subcommand = subcommand.to_owned();
        let Some(command) = self._find_command(&subcommand) else {
            println!(
                "error: No such command '{}', try '{} --help' for help.",
                subcommand, self.prog
            );
            if self.show_help_on_fail {
                self.print_help();
            }
            return;
        };

        for (key, value) in &parsed_flags {
            let canonical = command
                .known_flags
                .iter()
                .find(|f| f.alias.as_deref() == Some(key.as_str()))
                .map(|f| f.name.clone())
                .unwrap_or_else(|| key.clone());
            flags.insert(canonical, value.clone());
        }

        if flags.contains_key("help") {
            if !subcommand.is_empty() {
                match &command.usage {
                    Some(usage) => println!(
                        "Usage: {} {} {}\n{}",
                        self.prog, command.name, usage, command.description
                    ),
                    None => println!(
                        "{} {} - {}\nNo usage information available.",
                        self.prog, command.name, command.description
                    ),
                }
                return;
            }
            self.print_help();
            return;
        }

        if flags.contains_key("version") {
            println!("{} v{}", self.name, self.version);
            return;
        }

        for parsed_flag in flags.keys() {
            if parsed_flag == "help" || parsed_flag == "version" {
                continue;
            }
            let is_known = command.known_flags.iter().any(|f| f.name == *parsed_flag);
            if !is_known {
                if command.strict_flags {
                    println!(
                        "error: Unknown flag '--{}' for command '{}'.",
                        parsed_flag, subcommand
                    );
                    return;
                }
                println!(
                    "warning: Unknown flag '--{}' for command '{}'.",
                    parsed_flag, subcommand
                );
            }
        }

        let mut positionals = Vec::new();
        let mut skip_next = false;
        for arg in &args[1..] {
            if skip_next {
                skip_next = false;
                continue;
            }
            if arg.starts_with("--") {
                skip_next = true;
                continue;
            }
            if arg.starts_with('-') {
                continue;
            }
            positionals.push(arg.clone());
        }

        (command.handler)(&CommandContext {
            subcommand,
            positionals,
            flags: flags.clone(),
        });
    }
}

// FUTURE FEATURES? //
// after 0.1.0 (base):
// - commands use their docstring as the description?
