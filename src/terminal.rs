use crate::utils::input;

pub struct Terminal;

impl Terminal {
    pub fn prompt(prompt: &str) -> String {
        input(&(prompt.to_owned() + " "))
    }

    pub fn confirm(prompt: &str, default: Option<bool>, show_default: Option<bool>) -> bool {
        let default = default.is_some_and(|v| v);
        let show_default = show_default.is_none_or(|v| v);

        let suffix = if show_default {
            match default {
                true => " [Y/n]:",
                false => " [y/N]:",
            }
        } else {
            ""
        };

        loop {
            let choice = Terminal::prompt(&(prompt.to_owned() + suffix))
                .trim()
                .to_lowercase();
            match choice.as_str() {
                "" => return default,
                "yes" | "y" => return true,
                "no" | "n" => return false,
                _ => println!("Invalid input. Please answer y/n (Enter=default)"),
            }
        }
    }

    pub fn choice(
        prompt: &str,
        choices: &[&str],
        default: Option<&str>,
        show_default: Option<bool>,
        show_choices: Option<bool>,
    ) -> String {
        let show_choices = show_choices.is_none_or(|v| v);
        let show_default = show_default.is_none_or(|v| v);
        let default = default.map_or("", |v| v);

        if choices.is_empty() {
            panic!("choices cannot be empty");
        }
        if !default.is_empty() && !choices.contains(&default) {
            panic!("default '{}' is not in choices", default);
        }

        let suffix = " [".to_owned()
            + &if show_choices {
                let mut suffix = String::new();
                for (i, choice) in choices.iter().enumerate() {
                    suffix += &match default {
                        "" => choice.to_string(),
                        _ => {
                            if *choice == default && show_default {
                                format!("{}*", choice)
                            } else {
                                choice.to_string()
                            }
                        }
                    };
                    if i < choices.len() - 1 {
                        suffix += "/";
                    }
                }
                suffix
            } else {
                String::new()
            }
            + "]:";

        loop {
            let choice = Terminal::prompt(&(prompt.to_owned() + &suffix))
                .trim()
                .to_lowercase();
            if choices.iter().any(|c| c.to_lowercase() == choice) {
                return choice;
            } else if !default.is_empty() && choice.is_empty() {
                return default.to_string();
            }
            println!("Invalid input. Please answer one of the choices.");
        }
    }
}
