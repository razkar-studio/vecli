use vecli::*;

fn hello(ctx: &CommandContext) {
    if !ctx.flags.contains_key("silent") {
        println!("Hello!")
    }
}

fn main() {
    App::new("my-app")
        .name("My App")
        .description("a very informative description")
        .print_help_if_no_args(true)
        .print_help_on_fail(true)
        .version("0.1.2")
        .add_command(
            Command::new("hello", hello)
                .description("prints hello and exit.")
                .usage("[none]")
                .flag(Flag::new("silent").alias("s").description("Not say hello."))
                .strict_flags(true),
        )
        .run();
}
