use std::io;
use std::io::{Write};
use crimson6502::{CPU, Memory};
use crate::command::{CommandResult, Signal};
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
        loop {
            if let Some(input_vec) = self.prompt() {
                let result: CommandResult = CommandParser::parse(&input_vec);
                self.execute_result(result);
            };
        }
    }

    pub fn prompt(&mut self) -> Option<Vec<String>> {
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

    fn execute_result(&mut self, command_result: CommandResult) {
        match command_result {
            CommandResult::None => return,
            CommandResult::Message(message) => println!("{}", message),
            CommandResult::Signal(signal) => match signal {
                Signal::None => return,
                Signal::Quit => self.quit = true,
                Signal::CPUStep(n) => self.execute_cpu_step(n),
            }
        }
    }

    fn execute_cpu_step(&mut self, n: isize) {
        if self.cpu_ready() {
            for _ in 0..n {
                self.cpu
                    .as_mut()
                    .unwrap()
                    .run(
                        self
                        .memory
                        .as_mut()
                        .unwrap()
                    );
            }
        }
    }

    fn cpu_ready(&self) -> bool {
        self.cpu.is_some() && self.memory.is_some()
    }
}