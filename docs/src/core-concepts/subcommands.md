# Subcommands

Full subcommand chaining support was added on [version 0.3.0](https://github.com/razkar-studio/vecli/releases/tag/v0.3.0).

Say we want a `goodbye` subcommand for our `hello` command that prints a farewell message. We can achieve this flawlessly by adding a new subcommand to our `hello` command.

First, make the handler for it, we'll need it later:

```rust
// use vecli::*;

fn goodbye(_ctx: &CommandContext) {
    println!("Hello!");
    println!("Goodbye!");
}
```

To add this as a subcommand for our `hello` command, we can do it like so by using the `.subcommand()` method:

```rust
// let hello_command = Command::new(...)
// ...

hello_command.subcommand(
    Command::new("goodbye", goodbye)
);
```

Now, when the user runs `my-app hello goodbye` (or `cargo run hello goodbye`), the `goodbye` subcommand will be executed and print a farewell message!

```shell
$ my-app hello goodbye
Hello!
Goodbye!
```

Since subcommands are just regular commands, they can have their own flags, and even subcommands of their own. Configuring them is the same as configuring any other command.

## Extra

If you want a command that does nothing and is only a container for subcommands, you can use a separate `Command` constructor using `Command::parent("name")`. This is useful for organizing related subcommands together. Running a parent command without a subcommand will print an error by default, so it's best to pass `.print_help_if_no_args(true)` to make it print the help message instead.

```rust
let parent_command = Command::parent("parent").print_help_if_no_args(true);
```

---

Next up, let's implement the `silent` flag for our `hello` command. We can accomplish this by adding [Flags](./flags.md)
