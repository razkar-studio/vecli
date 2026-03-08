use vecli::*;

fn hello(_: &CommandContext) {
    println!("Hello!")
}

fn main() {
    App::new("my-app")
        .name("My App")
        .description("My App's Description")
        .add_command(Command::new("hello", hello))
        .run();
}
