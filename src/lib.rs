//! A zero-dependency, minimal CLI framework that's genuinely readable, the tool you've been looking for.
//!
//! # Getting Started
//!
//! Let's make your first CLI app using vecli.
//! First, Add vecli to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vecli = "0.1"
//! ```
//!
//! # Building Your First App
//!
//! Every vecli app starts with [`App`], registers [`Command`]s, and calls `.run()`:
//!
//! ```no_run
//! use vecli::{App, Command, CommandContext};
//!
//! fn hello(_: &CommandContext) {
//!     println!("Hello, world!");
//! }
//!
//! fn main() {
//!     App::new("myapp")
//!         .name("My App")
//!         .description("Does something cool.")
//!         .version("1.0.0")
//!         .show_help_if_no_args(true)
//!         .add_command(
//!             Command::new("hello", hello)
//!                 .description("Prints a greeting")
//!         )
//!         .run();
//! }
//! ```
//!
//! # Flags and Aliases
//!
//! Register flags on a command using [`Flag`], with optional short aliases:
//!
//! ```no_run
//! use vecli::{Command, CommandContext, Flag};
//!
//! fn install(ctx: &CommandContext) {
//!     let global = ctx.flags.get("global").is_some();
//!     println!("Installing... global={global}");
//! }
//!
//! Command::new("install", install)
//!     .flag(Flag::new("global").alias("g").description("Install globally"));
//! ```
//!
//! # Prompts
//!
//! vecli also ships [`Terminal`], [`Confirm`], and [`Choice`] for interactive input:
//!
//! ```no_run
//! use vecli::Confirm;
//!
//! let proceed = Confirm::new("Are you sure?")
//!     .default(false)
//!     .ask();
//! ```
//!
//! # Philosophy
//!
//! vecli is intentionally minimal, zero dependencies, and easy to understand.
//! If you need proc-macro attributes, automatic type coercion, or async handlers,
//! consider [`clap`](https://docs.rs/clap) instead. If you want something you can
//! read and understand in an afternoon, you're in the right place.
mod app;
mod terminal;
mod utils;

pub use app::App;
pub use app::Command;
pub use app::CommandContext;
pub use app::Flag;

pub use terminal::Choice;
pub use terminal::Confirm;
pub use terminal::Terminal;
