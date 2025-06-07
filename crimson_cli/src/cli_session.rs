use std::io;
use std::io::{Write, Read};
use std::fs;
use std::time::Instant;
use crimson6502::{CPU, Memory, CPUState};
use crate::command::{CommandResult, Signal};
use crate::command_parser::CommandParser;

pub struct CLISession {
    pub quit: bool,
    cpu: Option<CPU>,
    memory: Option<Memory>,
    file_contents: Option<Vec<u8>>,
}

impl CLISession {
    pub fn new() -> CLISession {
        Self {
            quit: false,
            cpu: None,
            memory: None,
            file_contents: None
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

    fn read_file(&mut self, path: &str) {
        if let Ok(mut file) = fs::File::open(path.to_string() + ".txt") {
            let mut buffer =  String::new();
            file.read_to_string(&mut buffer);
            let x = buffer.split_whitespace().map(|x: &str| u8::from_str_radix(x, 16)).collect();
            let vec: Vec<u8> = match x {
                Ok(v) => v,
                Err(err) => panic!("{:?}", err),
            };
            self.file_contents = Some(vec);
        } else {
            panic!("failed to open file: {}", path);
        }
    }

    fn run_file(&mut self) {
        for (i, value) in self.file_contents.clone().unwrap().into_iter().enumerate() {
            self.cpu.as_mut().unwrap().write_byte(i as u16, value);
        }
    }

    fn execute_result(&mut self, command_result: CommandResult) {
        match command_result {
            CommandResult::None => (),
            CommandResult::Message(message) => println!("{}", message),
            CommandResult::Signal(signal) => match signal {
                Signal::Quit 
                    => self.quit = true,
                Signal::CPUStep(n) if self.cpu_ready() 
                    => {
                        let now = Instant::now();
                        self.execute_cpu_step(n);
                        println!("Elapsed: {:?}", now.elapsed());
                    },
                Signal::CPUStep(_) 
                    => println!("CPU and/or Memory have not been initialized."),
                Signal::InitCPU
                    => self.cpu = Some(CPU::new(CPUState::new(), Memory::new())),
                Signal::InitMemory
                    => self.memory = Some(Memory::new()),
                Signal::InitAll
                    => {
                        self.cpu = Some(CPU::new(CPUState::new(), Memory::new()));
                        self.memory = Some(Memory::new());
                    },
                Signal::WriteMemory(addr, value) if self.memory.is_some() 
                    => self.cpu.as_mut().unwrap().write_byte(addr, value),
                Signal::WriteMemory(_, _)
                    => println!("Cannot write to Memory, it has not been initialized."),
                Signal::FileOpen(path)
                    => self.read_file(&path),
                Signal::FileRun
                    => self.run_file(),
            }
        }
    }

    fn execute_cpu_step(&mut self, n: isize) {
        for _ in 0..n {
            self.cpu
                .as_mut()
                .unwrap()
                .run();
        }
    }

    fn cpu_ready(&self) -> bool {
        self.cpu.is_some() && self.memory.is_some()
    }
}