use vecli::*;

fn hello(_: &CommandContext) {
    println!("Hello!")
}

fn main() {
    App::new("my-app")
        .name("My App")
        .description("a very informative description")
        .add_command(
            Command::new("hello", hello)
                .description("prints hello and exit.")
                .usage("[none]"),
        )
        .run();
}
