use vecli::*;

fn hello(ctx: &CommandContext) {
    if ctx.flags.contains_key("debug") {
        println!(
            "Debug Mode\nCommandContext {{ subcommand: '{}', positionals: {:?}, flags: {:?} }}",
            ctx.subcommand, ctx.positionals, ctx.flags
        );
        println!();
    }

    if ctx.flags.contains_key("dry-run") {
        println!("[DRY RUN] Would've greeted you with hello.");
    } else if !ctx.flags.contains_key("silent") {
        println!("Hello!")
    }
}

fn goodbye(ctx: &CommandContext) {
    if ctx.flags.contains_key("debug") {
        println!(
            "Debug Mode\nCommandContext {{ subcommand: '{}', positionals: {:?}, flags: {:?} }}",
            ctx.subcommand, ctx.positionals, ctx.flags
        );
        println!();
    }

    if ctx.flags.contains_key("dry-run") {
        println!("[DRY RUN] Would've greeted you with hello.");
    } else if !ctx.flags.contains_key("silent") {
        println!("Hello!")
    }
    if ctx.flags.contains_key("dry-run") {
        println!("[DRY RUN] Would've greeted you with goodbye.");
    } else if !ctx.flags.contains_key("silent") {
        println!("Goodbye!")
    }
}

fn entry(flags: PassedFlags) {
    println!(
        "Flags: {}",
        flags
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn main() {
    App::new("my-app")
        .main(entry)
        .name("My App")
        .description("a very informative description")
        .flag(
            Flag::global("debug")
                .alias("g")
                .description("shows debug information"),
        )
        .print_help_on_fail(true)
        .version("0.2.0")
        .add_command(
            Command::new("hello", hello)
                .description("prints hello and exit.")
                // don't need this anymore since command fallbacks properly
                // .usage("[options]")
                .flag(Flag::new("silent").alias("s").description("Not say hello."))
                .flag(
                    Flag::new("dry-run")
                        .alias("n")
                        .description("Run without making any changes."),
                )
                .strict_flags(true)
                .subcommand(
                    Command::new("goodbye", goodbye).description("Prints both hello and goodbye"),
                )
                .print_help_if_no_args(true),
        )
        .run();
}
