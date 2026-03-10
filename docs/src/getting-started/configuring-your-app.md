# Configuring Your App

Configuring your app is awfully simple, there's a handful of options you can configure for your app.

Here are the available options:

* **`.name("My App")`**: The human readable name for your app
* **`.description("the most awesome app ever")`**: The description for your app
* **`.version("0.1.0")`**: The version of your app
* `.print_help_if_no_args(true)`: Prints the help screen when no command is specified.
* `.print_help_on_fail(true)`: Prints help when no command found.

*(Added on newer versions, 0.2.0 or above)*
* **`.flag(Flag)`**: Adds a flag to your app, we'll get to this later.
* **`.strict_flags(true)`**: Enforces that only flags specified in `.flag(Flag)` are allowed.
* **`.main(fallback /* fn(PassedFlags) */)`**: Sets a fallback function to run when no command is specified. Conventional names: `entrypoint`, `entry`, `fallback`.


## Example

```rust
use vecli::*;

fn main() {
    App::new()
        .name("My App")
        .description("the most awesome app ever")
        .version("0.3.0")
        .print_help_if_no_args(true)
        .print_help_on_fail(true)
        .run();
}
```

---

Now that the app is configured how you want it, let's add some functionality to it, by [Adding Commands](../core-concepts/commands.md).
