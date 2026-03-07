use crate::utils::parse_flags;

pub struct CommandContext {
    pub subcommand: String,
    pub positionals: Vec<String>,
    pub flags: std::collections::HashMap<String, String>,
}

#[derive(Default)]
pub struct App {
    prog: &'static str,
    name: &'static str,
    description: &'static str,
    version: &'static str,
    commands: Vec<Command>,
    show_help_if_no_args: bool,
    show_help_on_fail: bool,
}

pub struct Command {
    name: &'static str,
    description: &'static str,
    usage: Option<&'static str>,
    handler: fn(&CommandContext),
}

impl App {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn prog(mut self, prog_name: &'static str) -> Self {
        self.prog = prog_name;
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn description(mut self, desc: &'static str) -> Self {
        self.description = desc;
        self
    }

    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
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

    pub fn add_command(
        mut self,
        name: &'static str,
        description: &'static str,
        handler: fn(&CommandContext),
        usage: Option<&'static str>,
    ) -> Self {
        self.commands.push(Command {
            name,
            description,
            handler,
            usage,
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
        let flags = parse_flags(&args);

        // THOUGHT PAUSE: A thought that doesn't need to be in a separate file  //
        // Why check this first?                                                //
        // - subcommand would print their error if it's first, and              //
        //   the contains check depends on subcommand already being defined.    //
        // Why not just use args[0]?                                            //
        // - Uhh, I don't know.                                                 //
        // RESOLVED                                                             //

        if args.len() == 0 && self.show_help_if_no_args {
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

        // ROADBLOCK! while implementing board #2
        // we don't actually know which command `subcommand` is yet.
        // OPTION! In app: move that for loop here/above and save as variable (CHANGES: derive(Copy) to Command)
        // OPTION! Extract: move that for loop in `utils.rs` as find_command(item: &str, in: Vec<Command>) and return vec_item that matches.
        // RESOLVED: OPTION! NEW! Internal helper

        if flags.contains_key("help") {
            if !subcommand.is_empty() {
                match command.usage {
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

        (command.handler)(&CommandContext {
            subcommand,
            positionals: args[1..].to_vec(),
            flags: flags.clone(),
        });
        return;
    }
}
