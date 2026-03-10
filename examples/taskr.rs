//! vecli example, a toy task manager CLI.
//!
//! Demonstrates all vecli features: commands, subcommands, global flags,
//! per-command flags, strict flags, positionals, Terminal prompts, and
//! the app-level main entry point.
//!
//! Run with:
//!   cargo run -- --help
//!   cargo run -- add "buy milk" --priority high
//!   cargo run -- list
//!   cargo run -- list urgent
//!   cargo run -- done "buy milk"
//!   cargo run -- clear

use vecli::*;

// --- Handlers ---

fn config_set(ctx: &CommandContext) {
    let key = ctx
        .positionals
        .first()
        .map(String::as_str)
        .unwrap_or("<key>");
    let value = ctx
        .positionals
        .get(1)
        .map(String::as_str)
        .unwrap_or("<value>");

    if ctx.flags.contains_key("verbose") {
        println!(
            "[verbose] config set called with key='{}', value='{}'",
            key, value
        );
    }

    println!("Set config '{}' to '{}'.", key, value);
}

fn config_show(ctx: &CommandContext) {
    if ctx.flags.contains_key("verbose") {
        println!("[verbose] config show called.");
    }

    println!("Current config:");
    println!("  theme   = dark");
    println!("  sort-by = priority");
}

fn add(ctx: &CommandContext) {
    // Demonstrates: positionals, per-command flags, strict flags, Choice prompt
    let task = ctx
        .positionals
        .first()
        .map(String::as_str)
        .unwrap_or("<unnamed>");

    let priority = if let Some(p) = ctx.flags.get("priority") {
        p.clone()
    } else {
        // If --priority not passed, ask interactively
        Choice::new("Priority:", &["low", "medium", "high"])
            .default("medium")
            .ask()
    };

    if ctx.flags.contains_key("verbose") {
        println!(
            "[verbose] add called with task='{}', priority='{}'",
            task, priority
        );
    }

    println!("Added task '{}' with priority {}.", task, priority);
}

fn list(ctx: &CommandContext) {
    // Demonstrates: global flag, per-command flag
    if ctx.flags.contains_key("verbose") {
        println!("[verbose] listing tasks...");
    }

    if ctx.flags.contains_key("all") {
        println!("Tasks (all):");
        println!("  [ ] buy milk       — high");
        println!("  [x] write docs     — medium");
        println!("  [ ] fix that bug   — low");
    } else {
        println!("Tasks (pending):");
        println!("  [ ] buy milk       — high");
        println!("  [ ] fix that bug   — low");
    }
}

fn list_urgent(ctx: &CommandContext) {
    // Demonstrates: subcommand handler
    if ctx.flags.contains_key("verbose") {
        println!("[verbose] listing urgent tasks...");
    }

    println!("Urgent tasks:");
    println!("  [ ] buy milk       — high");
}

fn done(ctx: &CommandContext) {
    // Demonstrates: positionals, global flag
    let task = ctx
        .positionals
        .first()
        .map(String::as_str)
        .unwrap_or("<unnamed>");

    if ctx.flags.contains_key("verbose") {
        println!("[verbose] marking '{}' as done...", task);
    }

    println!("Marked '{}' as done.", task);
}

fn clear(ctx: &CommandContext) {
    // Demonstrates: Confirm prompt, global flag
    if ctx.flags.contains_key("verbose") {
        println!("[verbose] clear called.");
    }

    let confirmed = Confirm::new("Clear all tasks?").default(false).ask();

    if confirmed {
        println!("All tasks cleared.");
    } else {
        println!("Aborted.");
    }
}

fn entry(flags: PassedFlags) {
    // Demonstrates: app-level main entry point with PassedFlags
    if flags.contains_flag("verbose") {
        println!("taskr: verbose mode active.");
    } else {
        println!("taskr: a toy task manager. Try 'taskr --help'.");
    }
}

// --- Main ---

fn main() {
    App::new("taskr")
        .name("Taskr")
        .description("A toy task manager. Demonstrates all vecli features.")
        .version("0.1.0")
        .main(entry)
        .flag(
            Flag::global("verbose")
                .alias("v")
                .description("Enable verbose output."),
        )
        .print_help_on_fail(true)
        .add_command(
            Command::new("add", add)
                .description("Add a new task.")
                .usage("<task> [--priority <level>]")
                .flag(
                    Flag::new("priority")
                        .alias("p")
                        .description("Task priority: low, medium, or high."),
                )
                .strict_flags(true),
        )
        .add_command(
            Command::new("list", list)
                .description("List pending tasks.")
                .flag(
                    Flag::new("all")
                        .alias("a")
                        .description("Include completed tasks."),
                )
                .subcommand(
                    Command::new("urgent", list_urgent)
                        .description("List only high-priority tasks."),
                ),
        )
        .add_command(
            Command::new("done", done)
                .description("Mark a task as done.")
                .usage("<task>"),
        )
        .add_command(
            Command::parent("config")
                .description("Manage taskr configuration.")
                .print_help_if_no_args(true)
                .subcommand(
                    Command::new("set", config_set)
                        .description("Set a config value.")
                        .usage("<key> <value>"),
                )
                .subcommand(Command::new("show", config_show).description("Show current config.")),
        )
        .add_command(Command::new("clear", clear).description("Clear all tasks."))
        .run();
}
