use std::io;
use std::io::{Write};
use crimson6502::{CPU, Memory, CPUState};
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
        while !self.quit {
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
        if let Err(_) = io::stdin().read_line(&mut input) {
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
            CommandResult::None => (),
            CommandResult::Message(message) => println!("{}", message),
            CommandResult::Signal(signal) => match signal {
                Signal::Quit 
                    => self.quit = true,
                Signal::CPUStep(n) if self.cpu_ready() 
                    => self.execute_cpu_step(n),
                Signal::CPUStep(_) 
                    => println!("CPU and/or Memory have not been initialized."),
                Signal::InitCPU
                    => self.cpu = Some(CPU::new(CPUState::new())),
                Signal::InitMemory
                    => self.memory = Some(Memory::new()),
                Signal::InitAll
                    => {
                        self.cpu = Some(CPU::new(CPUState::new()));
                        self.memory = Some(Memory::new());
                    },
                Signal::WriteMemory(addr, value) if self.memory.is_some() 
                    => self.memory.as_mut().unwrap().write_byte(addr, value),
                Signal::WriteMemory(_, _)
                    => println!("Cannot write to Memory, it has not been initialized.")
            }
        }
    }

    fn execute_cpu_step(&mut self, n: isize) {
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

    fn cpu_ready(&self) -> bool {
        self.cpu.is_some() && self.memory.is_some()
    }
}