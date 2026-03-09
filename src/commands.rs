//! Command definitions and the [`CommandContext`] type.
//!
//! [`Command`] represents a single subcommand registered on an [`crate::App`].
//! [`CommandContext`] is the parsed invocation context delivered to every handler.

use crate::App;
use crate::flags::Flag;

/// A single registered subcommand.
///
/// Build with [`Command::new`] and configure via the builder methods before
/// passing to [`App::add_command`].
pub struct Command {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) known_flags: Vec<Flag>,
    pub(crate) usage: Option<String>,
    pub(crate) handler: fn(&CommandContext),
    pub(crate) strict_flags: bool,
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
    ///
    /// If omitted and the command has registered flags, a fallback of `[options]` is shown.
    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Controls whether unknown flags cause a hard error or just a warning.
    ///
    /// When `true`, passing an unrecognized flag prints an error and exits without
    /// calling the handler. When `false` (the default), a warning is printed and
    /// execution continues. Global flags are always considered known and never
    /// trigger this check.
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

    /// Prints help text for this command to stdout.
    ///
    /// Output includes the usage line, description, and a formatted flag listing.
    /// If no usage string was set but flags are registered, a `[options]` fallback
    /// is used for the usage line.
    pub(crate) fn print_help(&self, app: &App) {
        if let Some(usage) = &self.usage {
            println!("Usage: {} {} {}", app.prog, self.name, usage);
        } else if !self.known_flags.is_empty() {
            // fallback that still makes sense
            println!("Usage: {} {} [options]", app.prog, self.name);
        }
        println!("    {}", self.description);
        println!();
        if !self.known_flags.is_empty() {
            println!("Options:");

            let longest = self
                .known_flags
                .iter()
                .map(|f| {
                    let alias_part = f.alias.as_ref().map_or(0, |a| a.len() + 4);
                    f.name.len() + alias_part
                })
                .max()
                .unwrap_or(0);

            for flag in &self.known_flags {
                let left = if let Some(alias) = &flag.alias {
                    format!("--{}, -{}", flag.name, alias)
                } else {
                    format!("--{}", flag.name)
                };
                let description = flag.description.as_deref().unwrap_or("");
                println!("    {:<width$} {}", left, description, width = longest + 10);
            }
        }
    }
}

/// Holds the parsed context for a command invocation.
///
/// Passed by reference to every command handler. Contains the resolved subcommand
/// name, any positional arguments (non-flag tokens), and the full set of flags
/// after alias resolution. Global flags registered on the app are merged in
/// alongside command-specific flags.
pub struct CommandContext {
    /// The subcommand name as typed by the user.
    pub subcommand: String,
    /// Positional arguments, in order, with flags filtered out.
    pub positionals: Vec<String>,
    /// Resolved flags, keyed by canonical name. Boolean flags have the value `"true"`.
    /// Includes both command-specific flags and any global flags that were passed.
    pub flags: std::collections::HashMap<String, String>,
}
