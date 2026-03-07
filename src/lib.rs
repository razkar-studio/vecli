/// A zero-dependency, bare-bones CLI framework that's genuinely readable.
mod app;
mod terminal;
mod utils;

pub use app::App;
pub use app::Command;
pub use app::CommandContext;

pub use terminal::Terminal;
