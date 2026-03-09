//! A zero-dependency, minimal CLI framework that's genuinely readable, the tool you've been looking for.
//!
//! Do you want to make a minimal CLI app but don't want a billion dependencies or doing all by yourself?
//! Then vecli is perfect for you! If not? You're at the wrong place.
//!
//! # Getting Started
//!
//! ## Installation
//!
//! To install `vecli` as a dependency, run:
//!
//! ```bash
//! cargo add vecli
//! ```
//!
//! which is the same as including it in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! vecli = "0.2"
//! ```
//!
//! ## Building Your First App
//!
//! Let's build your first app with `vecli`. This will be a simple "hello world" app that prints "hello" when run.
//! After adding `vecli` to your dependencies, we can start by making a new `App`.
//!
//! ```ignore
//! use vecli::*;
//!
//! fn main() {
//!     App::new("my-app")
//!         .run();
//! }
//! ```
//!
//! When you run the app, you should see something like:
//!
//! ```ignore
//! error: No command provided. Try 'my-app --help'.
//! ```
//!
//! And when you run `--help` (or `cargo run -- --help`), you should see a usage message like this:
//!
//! ```ignore
//! Usage: my-app <command> [options]
//!
//! No commands available. Add some using .add_command()!
//! ```
//!
//! ## Configuring Your App
//!
//! There's a handful of configuration options available for your app to further customize and to make your app look professional, listed:
//!
//! * **`.name("My App")`**: The human readable name for your app
//! * **`.description("the most awesome app ever")`**: The description for your app
//! * **`.version("0.1.0")`**: The version of your app
//! * `.print_help_if_no_args(true)`: Prints the help screen when no command is specified.
//! * `.print_help_on_fail(true)`: Prints help when no command found.
//!
//! ## Adding Commands
//!
//! Now let's make the app print "Hello" when the user passes in `hello` as a command. We'll want this:
//!
//! ```bash
//! cargo run hello # would print:
//! # Hello!
//! ```
//!
//! Vecli makes this awfully simple. Use `add_command()` that takes a `Command`, and you can construct them like this:
//!
//! ```ignore
//! app.add_command(
//!     Command::new("hello" /* The command the user would pass */, function /* The function that handles the command */)
//! )
//! ```
//!
//! And our `hello()` function:
//!
//! ```ignore
//! fn hello(_ctx: &CommandContext) {
//!     println!("Hello!");
//! }
//! ```
//!
//! ### Configuring Commands
//!
//! Like you can configure an App, you can also configure your command to make it professional.
//!
//! - **`.description("Prints hello and exit.")`**: The description of the command, what it does.
//! - **`.usage("[none]")`**: The usage for the command, will print alongside `my-app hello`.
//! - `.flag(Flag::new("silent"))`: Add a flag to the command, input is without the `--` prefix.
//! - `.strict_flags(true)`: If toggled, unknown flags will abort the program.
//!
//! ## Adding Flags
//!
//! Adding flags like `--silent` and `--dry-run` is also (kind of) simplified. To add a flag to your command, use the
//! previously mentioned `.flag()` method.
//!
//! To construct a flag, only the name is needed, but you can customize them with an alias and description:
//! - **`.alias("s")`**: An alias for the flag, in this case, `-s` will resolve to `--silent`. **CAUTION: Aliases will always return boolean!**
//! - **`.description("Not say hello.")`**: The description for the flag.
//!
//! ## CommandContext
//!
//! Saw that CommandContext class earlier? Good eye. Vecli gives the handler the context of the command, including:
//!
//! - `ctx.subcommand`: `hello` itself
//! - `ctx.positionals`: A vector of everything that comes after `subcommand`.
//! - `ctx.flags`: A `HashMap` of passed flags.
//!
//! It's completely up to the function to check and act based on what context was given by Vecli. Keep this in mind!
//!
//! ---
//!
//! And that's all you need to make a simple hello app using *vecli*! `--help` (even help for commands!) and `--version` is fully built-in to vecli.
//!
//! Here's all of them combined:
//!
//! # Examples
//! ```
//! use vecli::*;
//!
//! fn hello(ctx: &CommandContext) {
//!     if !ctx.flags.contains_key("silent") {
//!         println!("Hello!")
//!     }
//! }
//!
//! fn main() {
//!     App::new("my-app")
//!         .name("My App")
//!         .description("a very informative description")
//!         .print_help_if_no_args(true)
//!         .print_help_on_fail(true)
//!         .version("0.0.1")
//!         .add_command(
//!             Command::new("hello", hello)
//!                 .description("prints hello and exit.")
//!                 .usage("[none]")
//!                 .flag(Flag::new("silent").alias("s").description("Not say hello."))
//!                 .strict_flags(true),
//!         )
//!         .run();
//! }
//! ```
//!
//! If you want a feature added, please submit an issue to the [GitHub repository](https://github.com/razkar-studio/vecli.git)!
//! This project is still very experimental, so expect bugs. When you *do* find one, also submit an issue!
//! Feel free to read the rest of the documentation if you are a developer.
//!
//! Cheers, RazkarStudio.

mod app;
mod commands;
mod flags;

mod terminal;
mod utils;

pub use app::App;
pub use commands::Command;
pub use commands::CommandContext;
pub use flags::Flag;
pub use flags::PassedFlags;

pub use terminal::Choice;
pub use terminal::Confirm;
pub use terminal::Terminal;
