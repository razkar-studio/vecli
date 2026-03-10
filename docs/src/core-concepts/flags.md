# Flags

Flags are named options passed by the user at the command line, like `--silent` or `--output file.txt`.
vecli handles parsing, alias resolution, and delivery to your handler automatically.

## Defining a Flag

Flags are built with `Flag::new()` and attached to a command with `.flag()`:

```rust
Command::new("hello", function_that_handles_hello)
    .flag(Flag::new("silent").alias("s").description("Suppress output."))
```

The name is the canonical long form, without the `--` prefix. An alias is the short form, without `-`.

## Reading Flags in a Handler

Flags are available in `ctx.flags`, a `HashMap<String, String>`. Boolean flags have the value `"true"`.

```rust
fn greet(ctx: &CommandContext) {
    if ctx.flags.contains_key("silent") {
        return;
    }
    println!("Hello!");
}
```

Value-carrying flags (e.g. `--output file.txt`) store the value as a string:

```rust
let path = ctx.flags.get("output").map(String::as_str).unwrap_or("out.txt");
```

## Aliases

An alias lets users pass `-s` instead of `--silent`. vecli resolves it to the canonical name before
your handler is called, so you always check for `"silent"`, never `"s"`.

```rust
Flag::new("silent").alias("s")
```

> **Note:** Aliases are always boolean. `-s value` will not capture `value` as the flag's value.

## Global Flags

A global flag is available to every command without any extra setup. Define it on the app with `Flag::global()`:

```rust
App::new("mytool")
    .flag(Flag::global("verbose").alias("v").description("Enable verbose output."))
```

It shows up in every handler's `ctx.flags` automatically. Making a regular flag inside `App::flag()` means it's only available to the main entry (`.main(entry)`), if defined.

## Strict Mode

By default, unknown flags produce a warning and execution continues. Enable strict mode on a command
to treat unknown flags as a hard error instead:

```rust
Command::new("add", add)
    .strict_flags(true)
```

Strict mode can also be set at the app level via `App::strict_flags(true)`, which applies to app-level
flag parsing before any command is dispatched.

---

While making commands, you may have noticed that the function takes this peculiar argument: `&CommandContext`. What is that? Know more about the [CommandContext](./context.md) as our next stop.

If you've been this far from the beginning, congrats! This doc is under production and you've reached the end... for now. More guides will be added soon! For now, read the API reference at [docs.rs](https://docs.rs/vecli).
