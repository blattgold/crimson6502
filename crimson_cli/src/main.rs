mod cli_session;
mod command;
mod command_parser;

use std::io;
use crimson6502::{CPU, Memory, CPUState};
use crate::cli_session::CLISession;
use crate::command::Command;

fn main() {
    let mut session: CLISession = CLISession::new();
    session.run();
}