//! Low-level helpers used internally by the crate.
//!
//! [`input`] is a Python-style blocking read from stdin. [`parse_flags`] turns
//! a raw argument slice into a key-value flag map used by [`crate::app::App::run`].
//! [`format_flag`] produces a consistent display string for help output.

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
