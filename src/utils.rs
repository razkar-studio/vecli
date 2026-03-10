//! Low-level helpers used internally by the crate.
//!
//! [`input`] is a Python-style blocking read from stdin. [`parse_flags`] turns
//! a raw argument slice into a key-value flag map used by [`crate::app::App::run`].
//! [`format_flag`] produces a consistent display string for help output.

use crate::*;
use std::io::{self, Write};

/// Prints a prompt and reads one line from stdin.
///
/// Flushes stdout before reading so the prompt appears immediately, even in
/// environments that buffer output. The returned string has trailing newline
/// and carriage-return characters stripped, but leading whitespace is preserved.
///
/// # Panics
///
/// Panics if stdout cannot be flushed or if reading from stdin fails.
///
/// # Example
/// ```ignore
/// let name = input("Enter your name:");
/// println!("Hello, {name}!");
/// ```
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim_end().to_string()
}

/// Formats a flag name and optional alias into a help display string.
///
/// Produces `--name, -alias` when an alias is present, or `--name` when not.
/// Used to build consistent left-side columns in help output.
///
/// # Example
/// ```
/// // format_flag("silent", Some("s")) -> "--silent, -s"
/// // format_flag("verbose", None)     -> "--verbose"
/// ```
pub fn format_flag(name: &str, alias: Option<&str>) -> String {
    if let Some(alias) = alias {
        format!("--{}, -{}", name, alias)
    } else {
        format!("--{}", name)
    }
}

/// Parses a slice of argument strings into a flag map.
///
/// Recognizes two flag forms:
/// - Long flags (`--name`): may consume the next token as a value if it does
///   not start with `-`. Otherwise the value is `"true"`.
/// - Short flags (`-x`): always produce the value `"true"`. Value-carrying
///   short flags are not supported.
///
/// Tokens that do not start with `-`, or that start with `---`, are skipped.
/// The generic bound `S: AsRef<str>` lets you pass `&[String]` or `&[&str]`
/// without converting first.
///
/// # Example
/// ```ignore
/// let args = vec!["--verbose", "--output", "file.txt", "-q", "positional"];
/// let flags = parse_flags(&args);
/// assert_eq!(flags["verbose"], "true");
/// assert_eq!(flags["output"], "file.txt");
/// assert_eq!(flags["q"], "true");
/// assert!(!flags.contains_key("positional"));
/// ```
pub fn parse_flags<S: AsRef<str>>(args: &[S]) -> std::collections::HashMap<String, String> {
    let mut flags = std::collections::HashMap::new();
    let args: Vec<&str> = args.iter().map(|arg| arg.as_ref()).collect();

    let mut i = 0;
    while i < args.len() {
        let arg = args[i];

        if arg.starts_with("-") && !arg.starts_with("---") {
            let flag_name = if let Some(stripped) = arg.strip_prefix("--") {
                stripped.to_string()
            } else {
                arg[1..].to_string()
            };

            if arg.starts_with("--") {
                // long flags can have values: --version 1.0
                if i + 1 < args.len() && !args[i + 1].starts_with("-") {
                    flags.insert(flag_name, args[i + 1].to_string());
                    i += 2;
                } else {
                    flags.insert(flag_name, "true".to_string());
                    i += 1;
                }
            } else {
                // short flags are always boolean: -h -> true
                flags.insert(flag_name, "true".to_string());
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    flags
}

/// Recursively dispatches a parsed invocation through the command tree.
///
/// Starting from `command`, scans `args` for the next positional token and
/// checks whether it matches a registered subcommand. If it does, resolves
/// any flag aliases against the subcommand's known flags and recurses with
/// the matched subcommand and the remaining args. Recursion continues until
/// no subcommand matches or `args` is exhausted, at which point the current
/// command's handler is called.
///
/// Before calling the handler, flags are validated against `command.known_flags`
/// and `global_flags`. If `command.strict_flags` is set, an unrecognized flag
/// aborts with an error message. Otherwise a warning is printed and execution
/// continues.
///
/// The [`CommandContext`] delivered to the handler always carries:
/// - `subcommand`: the name of the deepest matched command.
/// - `positionals`: non-flag tokens remaining after the deepest matched command name.
/// - `flags`: the resolved flag map at that depth, including global flags.
///
/// # Behavior when no subcommand is found
///
/// If no positional token is present in `args`:
/// - If `command` has a handler, it is called (with a warning if
///   `print_help_if_no_args` is also set, since they are mutually exclusive).
/// - If `command` has no handler and `print_help_if_no_args` is set, help is
///   printed via [`Command::print_help`].
/// - If neither is set, an error message is printed and the function returns.
pub(crate) fn dispatch(
    command: &Command,
    args: &[String],
    flags: std::collections::HashMap<String, String>,
    global_flags: &[Flag],
    prog: &str,
) {
    let next = args.iter().find(|a| !a.starts_with('-'));

    if let Some(name) = next
        && let Some(sub) = command.subcommands.iter().find(|s| s.name == *name)
    {
        let mut sub_flags = flags.clone();
        for (key, value) in flags.iter() {
            let canonical = sub
                .known_flags
                .iter()
                .chain(global_flags.iter())
                .find(|f| f.alias.as_deref() == Some(key.as_str()))
                .map(|f| f.name.clone())
                .unwrap_or_else(|| key.clone());
            sub_flags.insert(canonical, value.clone());
        }
        return dispatch(sub, &args[1..], sub_flags, global_flags, prog);
    }

    if next.is_none() {
        if command.handler.is_some() && command.print_help_if_no_args {
            eprintln!(
                "warning: handler and print_help_if_no_args are mutually exclusive. Handler takes priority."
            );
        }
        if command.handler.is_none() {
            if command.print_help_if_no_args {
                command.print_help(prog);
                return;
            } else {
                println!("error: No subcommand provided.");
                return;
            }
        }
    }

    for parsed_flag in flags.keys() {
        if parsed_flag == "help" || parsed_flag == "version" {
            continue;
        }
        let is_known = command
            .known_flags
            .iter()
            .chain(global_flags.iter())
            .any(|f| f.name == *parsed_flag);
        if !is_known {
            if command.strict_flags {
                println!(
                    "error: Unknown flag '--{}' for command '{}'.",
                    parsed_flag, command.name
                );
                return;
            }
            println!("warning: Unknown flag '--{}'.", parsed_flag);
        }
    }

    let positionals: Vec<String> = {
        let mut result = Vec::new();
        let mut skip_next = false;
        for arg in args.iter() {
            if skip_next {
                skip_next = false;
                continue;
            }
            if arg.starts_with("--") {
                let key = &arg[2..];
                if let Some(val) = flags.get(key) {
                    if val != "true" {
                        skip_next = true;
                    }
                }
                continue;
            }
            if arg.starts_with('-') {
                continue;
            }
            result.push(arg.clone());
        }
        result
    };

    if let Some(handler) = command.handler {
        handler(&CommandContext {
            subcommand: command.name.clone(),
            positionals,
            flags,
        });
    }
}
