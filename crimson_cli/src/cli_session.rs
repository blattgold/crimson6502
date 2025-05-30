use std::io;
use std::io::{Write};
use crimson6502::{CPU, Memory};
use crate::command::{Command, CommandComponent};
use crate::command_parser::CommandParser;

pub struct CLISession {
    pub quit: bool,
    cpu: Option<CPU>,
    memory: Option<Memory>,
}

impl CLISession {
    pub fn new() -> CLISession {
        Self {
            quit: false,
            cpu: None,
            memory: None,
        }
    }

    pub fn prompt(&mut self) -> Result<Command, String> {
        print!("Command> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            return Err(format!("Error while trying to get user input: {}", error));
        }
        input.trim().to_string();

        if input.len() == 0 {
            return Err(String::from(""))
        }

        let command: Command = CommandParser::parse(input);
        Ok(command)
    }

    fn cpu_ready(&self) -> bool {
        !(self.cpu.is_none() | self.memory.is_none())
    }
}