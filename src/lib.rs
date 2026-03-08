//! A zero-dependency, bare-bones CLI framework that's genuinely readable.
//!
//! Build an [`App`], register [`Command`]s, and call [`App::run`]. That's it.
//! Flags support short aliases, strict-mode validation, and automatic `--help`
//! and `--version` handling with no configuration required.
//!
//! # Example
//! ```
//! use vecli::{App, Command, CommandContext};
//!
//! fn hello(_: &CommandContext) {
//!     println!("Hello!")
//! }
//!
//! fn main() {
//!     let app = App::new("my-app")
//!         .name("My App")
//!         .description("My App's Description")
//!         .add_command(Command::new("hello", hello));
//!
//!     app.run();
//! }
//! ```
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
