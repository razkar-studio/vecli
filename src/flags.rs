//! Flag definitions and the [`PassedFlags`] context type.
//!
//! [`Flag`] describes a named option that can be attached to a [`crate::Command`]
//! or registered at the app level via [`crate::App::flag`]. [`PassedFlags`] carries
//! the resolved flag map delivered to a main entry handler.

/// A flag definition for a [`crate::Command`] or app-level registration.
///
/// Flags can carry an optional short alias (e.g. `"h"` for `"help"`) which is
/// resolved to the canonical name before the handler is called. Use [`Flag::global`]
/// to create a flag that is available across all commands.
#[derive(Default, Clone)]
pub struct Flag {
    pub(crate) is_global: bool,
    /// The canonical long name, without the `--` prefix.
    pub(crate) name: String,
    /// Optional short alias, without the `-` prefix.
    pub(crate) alias: Option<String>,
    /// Human-readable description shown in generated help text.
    pub(crate) description: Option<String>,
}

/// Resolved flags delivered to the app's main entry handler.
///
/// Constructed by [`crate::App::run`] when no subcommand is present and a main
/// entry point has been registered via [`crate::App::main`]. The inner map uses
/// canonical flag names as keys; boolean flags have the value `"true"`.
pub struct PassedFlags {
    /// The raw resolved flag map. Prefer the accessor methods over direct access.
    pub map: std::collections::HashMap<String, String>,
}

impl PassedFlags {
    /// Returns the value of a flag by its canonical name, if present.
    ///
    /// Boolean flags (those with no explicit value) return `"true"`.
    pub fn get_flag_value(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }

    /// Returns `true` if the named flag was passed by the user.
    pub fn contains_flag(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    /// Iterates over all `(name, value)` pairs in the resolved flag map.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.map.iter()
    }
}

impl Flag {
    /// Creates a new command-scoped flag with the given canonical name.
    ///
    /// The name should be given without the `--` prefix, e.g. `"silent"` for `--silent`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Creates a new global flag available to all commands in the app.
    ///
    /// Global flags are resolved and merged into [`crate::CommandContext::flags`]
    /// for every command, so handlers can read them without any extra setup.
    /// Register global flags on the app via [`crate::App::flag`].
    ///
    /// # Example
    /// ```
    /// use vecli::Flag;
    ///
    /// let debug = Flag::global("debug").alias("g").description("Enable debug output.");
    /// ```
    pub fn global(name: impl Into<String>) -> Self {
        Self {
            is_global: true,
            name: name.into(),
            ..Default::default()
        }
    }

    /// Sets the short alias for this flag (e.g. `"h"` to match `-h`).
    ///
    /// Aliases are always treated as boolean regardless of the long flag's value type.
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
