//! Interactive terminal prompt utilities.
//!
//! Provides [`Terminal`] for direct one-shot prompts, and the [`Confirm`]
//! and [`Choice`] builder types for cases where you want to configure a
//! prompt incrementally before running it.

use crate::utils::input;

/// Collection of interactive terminal prompt utilities.
///
/// All methods block on stdin until the user provides valid input.
/// Invalid responses print an inline error and re-prompt rather than returning
/// an error value, so callers always receive a valid result.
pub struct Terminal;

impl Terminal {
    /// Prints `prompt` and reads a line from stdin.
    ///
    /// A single trailing space is appended to `prompt` before printing so the
    /// cursor lands one character after the prompt text.
    pub fn prompt(prompt: &str) -> String {
        input(&(prompt.to_owned() + " "))
    }

    /// Asks a yes/no question and returns the user's answer as a `bool`.
    ///
    /// Re-prompts on any input that is not `y`, `yes`, `n`, `no`, or an empty
    /// string (which selects the default).
    ///
    /// # Arguments
    ///
    /// * `prompt` - The question text, without any trailing suffix.
    /// * `default` - The value returned when the user presses Enter with no input.
    ///   Defaults to `false` if `None`.
    /// * `show_default` - When `true` (or `None`), appends `[Y/n]` or `[y/N]`
    ///   to the prompt to indicate the default visually.
    pub fn confirm(prompt: &str, default: Option<bool>, show_default: Option<bool>) -> bool {
        let default = default.is_some_and(|v| v);
        let show_default = show_default.is_none_or(|v| v);

        let suffix = if show_default {
            match default {
                true => " [Y/n]:",
                false => " [y/N]:",
            }
        } else {
            ""
        };

        loop {
            let choice = Terminal::prompt(&(prompt.to_owned() + suffix))
                .trim()
                .to_lowercase();
            match choice.as_str() {
                "" => return default,
                "yes" | "y" => return true,
                "no" | "n" => return false,
                _ => println!("Invalid input. Please answer y/n (Enter=default)"),
            }
        }
    }

    /// Asks the user to pick one option from a fixed set of choices.
    ///
    /// Matching is case-insensitive. Re-prompts until the user enters a
    /// recognized choice or presses Enter when a default is set.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The question text.
    /// * `choices` - The allowed responses. Must not be empty.
    /// * `default` - Optional default value returned on empty input. Must be
    ///   present in `choices` if provided.
    /// * `show_default` - When `true` (or `None`), marks the default choice
    ///   with a `*` in the suffix list.
    /// * `show_choices` - When `true` (or `None`), appends the full choice list
    ///   to the prompt as `[a/b*/c]`.
    ///
    /// # Panics
    ///
    /// Panics if `choices` is empty, or if `default` is set to a value not
    /// present in `choices`.
    pub fn choice(
        prompt: &str,
        choices: &[&str],
        default: Option<&str>,
        show_default: Option<bool>,
        show_choices: Option<bool>,
    ) -> String {
        let show_choices = show_choices.is_none_or(|v| v);
        let show_default = show_default.is_none_or(|v| v);
        let default = default.map_or("", |v| v);

        if choices.is_empty() {
            panic!("choices cannot be empty");
        }
        if !default.is_empty() && !choices.contains(&default) {
            panic!("default '{}' is not in choices", default);
        }

        let suffix = " [".to_owned()
            + &if show_choices {
                let mut suffix = String::new();
                for (i, choice) in choices.iter().enumerate() {
                    suffix += &match default {
                        "" => choice.to_string(),
                        _ => {
                            if *choice == default && show_default {
                                format!("{}*", choice)
                            } else {
                                choice.to_string()
                            }
                        }
                    };
                    if i < choices.len() - 1 {
                        suffix += "/";
                    }
                }
                suffix
            } else {
                String::new()
            }
            + "]:";

        loop {
            let choice = Terminal::prompt(&(prompt.to_owned() + &suffix))
                .trim()
                .to_lowercase();
            if choices.iter().any(|c| c.to_lowercase() == choice) {
                return choice;
            } else if !default.is_empty() && choice.is_empty() {
                return default.to_string();
            }
            println!("Invalid input. Please answer one of the choices.");
        }
    }
}

/// Builder for a yes/no confirmation prompt.
///
/// Wraps [`Terminal::confirm`] with a chainable configuration API. Call
/// [`Confirm::ask`] to display the prompt and block until the user answers.
///
/// # Example
/// ```no_run
/// let confirmed = Confirm::new("Delete file?")
///     .default(false)
///     .ask();
/// ```
pub struct Confirm<'a> {
    prompt: &'a str,
    default: Option<bool>,
    show_default: bool,
}

impl<'a> Confirm<'a> {
    /// Creates a new `Confirm` with the given question text.
    ///
    /// Defaults to `false` when the user presses Enter, with the default indicator visible.
    pub fn new(prompt: &'a str) -> Self {
        Self {
            prompt,
            default: None,
            show_default: true,
        }
    }

    /// Sets the value returned when the user presses Enter with no input.
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    /// Controls whether `[Y/n]` or `[y/N]` is appended to the prompt text.
    pub fn show_default(mut self, show: bool) -> Self {
        self.show_default = show;
        self
    }

    /// Displays the prompt and returns the user's answer.
    pub fn ask(self) -> bool {
        Terminal::confirm(self.prompt, self.default, Some(self.show_default))
    }
}

/// Builder for a multiple-choice prompt.
///
/// Wraps [`Terminal::choice`] with a chainable configuration API. Call
/// [`Choice::ask`] to display the prompt and block until the user
/// selects a valid option.
///
/// # Example
/// ```no_run
/// let env = Choice::new("Select environment:", &["dev", "staging", "prod"])
///     .default("dev")
///     .ask();
/// ```
pub struct Choice<'a> {
    prompt: &'a str,
    choices: &'a [&'a str],
    default: Option<&'a str>,
    show_default: bool,
    show_choices: bool,
}

impl<'a> Choice<'a> {
    /// Creates a new `Choice` with the given question text and choice list.
    ///
    /// `choices` must not be empty. Both the default indicator and the inline
    /// choice list are shown by default.
    pub fn new(prompt: &'a str, choices: &'a [&'a str]) -> Self {
        Self {
            prompt,
            choices,
            default: None,
            show_default: true,
            show_choices: true,
        }
    }

    /// Sets the default choice returned when the user presses Enter with no input.
    ///
    /// Must be a value present in `choices`; validated at prompt time by [`Terminal::choice`].
    pub fn default(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    /// Controls whether the default choice is marked with `*` in the suffix list.
    pub fn show_default(mut self, show: bool) -> Self {
        self.show_default = show;
        self
    }

    /// Controls whether the full choice list is appended to the prompt as `[a/b/c]`.
    pub fn show_choices(mut self, show: bool) -> Self {
        self.show_choices = show;
        self
    }

    /// Displays the prompt and returns the user's selection as a lowercase string.
    pub fn ask(self) -> String {
        Terminal::choice(
            self.prompt,
            self.choices,
            self.default,
            Some(self.show_default),
            Some(self.show_choices),
        )
    }
}
