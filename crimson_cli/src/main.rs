mod cli_session;
mod command;
mod command_parser;

use std::io;
use crimson6502::{CPU, Memory, CPUState};
use crate::cli_session::CLISession;
use crate::command::Command;

fn main() {
    let mut session: CLISession = CLISession::new();

    let mut input = String::new();
    while !session.quit {
        let command_result: Result<Command, String> = session.prompt();

        if let Ok(command) = command_result {
            session.execute_command(command);
        } else if let Err(error) = command_result {
            println!("{}", error);
        }
    }
}