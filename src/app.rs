use crate::utils::parse_flags;

pub struct CommandContext {
    pub subcommand: String,
    pub positionals: Vec<String>,
    pub flags: std::collections::HashMap<String, String>,
}

#[derive(Default)]
pub struct App {
    prog: String,
    name: String,
    description: String,
    version: String,
    commands: Vec<Command>,
    show_help_if_no_args: bool,
    show_help_on_fail: bool,
}

pub struct Command {
    name: String,
    description: String,
    known_flags: Vec<Flag>,
    usage: Option<String>,
    handler: fn(&CommandContext),
    strict_flags: bool,
}

#[derive(Default)]
pub struct Flag {
    pub name: String,
    pub alias: Option<String>,
    pub description: Option<String>,
}

impl Flag {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

impl Command {
    pub fn new(name: impl Into<String>, handler: fn(&CommandContext)) -> Self {
        Self {
            name: name.into(),
            handler,
            description: "".into(),
            known_flags: Vec::new(),
            usage: None,
            strict_flags: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.usage = Some(usage.into());
        self
    }

    pub fn strict_flags(mut self, strict: bool) -> Self {
        self.strict_flags = strict;
        self
    }

    pub fn flag(mut self, flag: Flag) -> Self {
        self.known_flags.push(flag);
        self
    }
}

impl App {
    pub fn new(prog_name: impl Into<String>) -> Self {
        Self {
            prog: prog_name.into(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn show_help_if_no_args(mut self, show: bool) -> Self {
        self.show_help_if_no_args = show;
        self
    }

    pub fn show_help_on_fail(mut self, show: bool) -> Self {
        self.show_help_on_fail = show;
        self
    }

    fn _find_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn add_command_param(
        mut self,
        name: impl Into<String>,
        flags: Option<Vec<Flag>>,
        description: impl Into<String>,
        handler: fn(&CommandContext),
        usage: Option<impl Into<String>>,
        strict_flags: bool,
    ) -> Self {
        // Command says, usage: Option<String>
        // this usage is: usage: Option<impl Into<String>>
        // still fails..
        self.commands.push(Command {
            name: name.into(),
            description: description.into(),
            handler,
            usage: usage.map(|u| u.into()),
            known_flags: flags.unwrap_or_default(),
            strict_flags,
        });
        self
    }

    pub fn print_help(&self) {
        println!("{} v{}", self.name, self.version);
        println!("{}", self.description);
        println!();
        println!("USAGE:");
        println!("  {} <command> [options]", self.prog);
        println!();
        println!("COMMANDS:");
        for cmd in &self.commands {
            println!("  {:<15} {}", cmd.name, cmd.description);
        }
    }

    pub fn run(&self) {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let parsed_flags = parse_flags(&args);
        // Hmm.. how do we determine about aliases..
        // THINKING BOARD
        // parse_flags returns {'h': 'true'}
        // Flag::new("help").alias("h") // 'h' in self.alias!
        // loop through flags and command?
        // flag.name == command.alias?
        // > it's an alias! replace the entry or something (i dont know how to do this one)
        // resolved!
        let mut flags = std::collections::HashMap::new();

        if args.is_empty() && self.show_help_if_no_args {
            self.print_help();
            return;
        }

        let Some(subcommand) = args.first() else {
            println!("error: No command provided. Try '{} --help'.", self.prog);
            return;
        };
        let subcommand = subcommand.to_owned();
        let Some(command) = self._find_command(&subcommand) else {
            println!(
                "error: No such command '{}', try '{} --help' for help.",
                subcommand, self.prog
            );
            if self.show_help_on_fail {
                self.print_help();
            }
            return;
        };

        for (key, value) in &parsed_flags {
            let canonical = command
                .known_flags
                .iter()
                .find(|f| f.alias.as_deref() == Some(key.as_str()))
                .map(|f| f.name.clone())
                .unwrap_or_else(|| key.clone());
            flags.insert(canonical, value.clone());
        }

        if flags.contains_key("help") {
            if !subcommand.is_empty() {
                match &command.usage {
                    Some(usage) => println!(
                        "Usage: {} {} {}\n{}",
                        self.prog, command.name, usage, command.description
                    ),
                    None => println!(
                        "{} {} - {}\nNo usage information available.",
                        self.prog, command.name, command.description
                    ),
                }
                return;
            }
            self.print_help();
            return;
        }

        if flags.contains_key("version") {
            println!("{} v{}", self.name, self.version);
            return;
        }

        for parsed_flag in flags.keys() {
            if parsed_flag == "help" || parsed_flag == "version" {
                continue;
            }
            let is_known = command.known_flags.iter().any(|f| f.name == *parsed_flag);
            if !is_known {
                if command.strict_flags {
                    println!(
                        "error: Unknown flag '--{}' for command '{}'.",
                        parsed_flag, subcommand
                    );
                    return;
                }
                println!(
                    "warning: Unknown flag '--{}' for command '{}'.",
                    parsed_flag, subcommand
                );
            }
        }

        (command.handler)(&CommandContext {
            subcommand,
            positionals: args[1..]
                .iter()
                .filter(|a| !a.starts_with('-'))
                .cloned()
                .collect(),
            flags: flags.clone(),
        });
    }
}

// FUTURE FEATURES? //
// after 0.1.0 (base):
// - commands use their docstring as the description?
//
// SHOWCASE //
// // It's this easy.
// use vela::{App, CommandContext};
//
// fn hello(_: &CommandContext) {
//      println!("Hello!")
// }
//
// fn main() {
//      let app = App::new("my-app")
//          .name("My App")
//          .description("My App's Description")
//          .add_command(Command::new("hello", hello));
//
//      app.run();
// }
//
