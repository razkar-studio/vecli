use std::io::{self, Write};

/// Python-like input() with built-in error handling.
/// Returns a String instead of a Result.
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim_end().to_string()
}

pub fn parse_flags<S: AsRef<str>>(args: &[S]) -> std::collections::HashMap<String, String> {
    let mut flags = std::collections::HashMap::new();
    let args: Vec<&str> = args.iter().map(|arg| arg.as_ref()).collect();

    let mut i = 0;
    while i < args.len() {
        let arg = args[i];

        if arg.starts_with("--") && !arg.starts_with("---") {
            let flag_name = arg[2..].to_string();

            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                flags.insert(flag_name, args[i + 1].to_string());
                i += 2;
            } else {
                flags.insert(flag_name, "true".to_string());
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    flags
}
