# Commands

Let's give your app some commands to run. Here's how you can define them:

```rust
// use vecli::*;

let hello_command = Command::new("hello", function_that_runs_on_hello /* fn(&CommandContext) */);

// Adding this to the app:
// App::new() ...
    .add_command(hello_command)
// ... .run();
```

The above is a simple command that runs when the user types the command `hello`, as in `my-app hello`.

To actually make the command run, you need to define the function `function_that_runs_on_hello` that will be called when the command is executed.

```rust
fn function_that_runs_on_hello(_ctx: &CommandContext) {
    println!("Hello!");
}
```

Put two together, and you have a working app!

```sh
cargo run hello
```

That should print `Hello!` to the console.

A shorter way to do this is to just directly define `Command` inside `.add_command()`:

```rust
App::new()
    .add_command(Command::new("hello", function_that_runs_on_hello))
// ... .run();
```

## Configuration

Just like with the app itself, you can configure how the command is displayed on the help screen and usage output.

For example, you can set the command's description and usage text:

```rust
Command::new("hello", function_that_runs_on_hello)
    .description("Prints a friendly greeting")
    .usage("<none>"); // a suffix to `my-app hello`
```

This will display the command's description and usage text when the user runs `my-app help` or `my-app hello --help`.

The full configuration options for commands:
- **`.description("Prints hello and exit.")`**: The description of the command, what it does.
- **`.usage("[none]")`**: The usage for the command, will print alongside `my-app hello`.
- `.strict_flags(true)`: If toggled, unknown flags will abort the program. We'll get to flags in a moment.

*(Added in 0.3.0)*
- `.print_help_if_no_args(true)`: If toggled, the help screen will be printed if no arguments (subcommands) are provided.

---

Okay, that was a lot to take in. Take a breather and let's continue. Say we want a `goodbye` subcommand that prints a farewell message. That's what we'll cover next on [Subcommands]().
