<div align="center">

# vecli

**A zero-dep CLI framework that's genuinely readable.**

</div>

---

# Work In Progress
vecli is currently a work in progress and is not ready for use. The API is subject to change and the crate is not yet published.

# What is vecli?
vecli is a zero-dep CLI framework made in Rust with UX in mind, and makes development of CLI tools easy and straightforward.

# Usage

Let's create your first CLI tool using vecli.

Create a new Rust project and add vecli as a dependency. More details [here](#installation)

Open `main.rs` and add the following code:

```rust
use vecli::*;

fn hello(_: &CommandContext) {
     println!("Hello!")
}

fn main() {
     let app = App::new("my-app")
         .name("My App")
         .description("My App's Description")
         .add_command(Command::new("hello", hello))
         .run();
}
```

Run `cargo run hello`, and you should see `Hello!` printed to the console.
Congrats, you've created your first CLI tool using vecli! Really, it's *that* easy.

# Installation
To install vecli, add the following to your `Cargo.toml` file:

```toml
[dependencies]
vecli = { git = "https://github.com/razkar-studio/vecli.git" }
```

# License
This project is protected by the RazkarStudio Permissive License, a permissive source license with limitations to AI/ML training use. See [LICENSE.md](LICENSE.md) for more information.

# Contributing
Contributions are welcome! Please open an issue or submit a pull request on the [GitHub repository](https://github.com/razkar-studio/vecli).

Cheers, RazkarStudio.
