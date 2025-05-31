use std::io;
use std::io::{Write};
use crimson6502::{CPU, Memory};
use crate::command::{Command};
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

    pub fn run(&mut self) {
        let parser = CommandParser::new();
        loop {
            if let Some(input_vec) = self.prompt(&parser) {
                parser.parse(&input_vec);
            };
        }
    }

    pub fn prompt(&mut self, parser: &CommandParser) -> Option<Vec<String>> {
        print!("Command> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            return None;
        }
        let input = input.trim().to_string();

        if input.len() == 0 {
            return None;
        }

        let input_split: Vec<String> = input
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        Some(input_split)
    }

    fn cpu_ready(&self) -> bool {
        self.cpu.is_some() && self.memory.is_some()
    }
}