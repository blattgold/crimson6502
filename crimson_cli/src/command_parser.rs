use std::num::ParseIntError;

use crate::command::Command;

pub struct CommandParser;

impl CommandParser {
    pub fn parse(input: String) -> Command {
        let arguments: Vec<&str> = input.split_whitespace().collect();
        match arguments[0].to_lowercase().as_str() {
            "quit" => Command::Quit,
            "step" => Self::parse_args_step(arguments),
            other => Command::Unknown(String::from(other))
        }
    }

    fn parse_args_step(arguments: Vec<&str>) -> Command {
        if arguments.len() == 1 {
            return Command::Step(1)
        } else {
            let step_amount: Result<usize, ParseIntError> = arguments[1].parse();

            if let Ok(n) = step_amount {
                return Command::Step(n);
            } else {
                return Command::InvalidArgs(String::from(arguments[1]))
            }
        }
    }
}