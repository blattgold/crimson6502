use std::collections::HashMap;
use crate::command::{Command, GrammarToken, CommandResult};

pub struct CommandParser {
    registered_commands: HashMap<String, Command>,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        let mut parser: CommandParser = CommandParser {
            registered_commands: HashMap::<String, Command>::new(),
        };
        parser.register_default_commands();
        parser
    }

    pub fn register_command(&mut self, command: Command) {
        self.registered_commands.insert(command.repr_str.clone(), command);
    }

    fn register_default_commands(&mut self) {
        self.register_command(Command::new(
            1,
            String::from("quit"),
            vec![GrammarToken::Command],
            || CommandResult::SignalQuit,
        ));
    }

    pub fn parse(&self, input_vec: &[String]) -> Option<Command> {
        if input_vec.is_empty() {
            return None;
        }
        self.registered_commands.get(&input_vec[0]).cloned()
    }
}