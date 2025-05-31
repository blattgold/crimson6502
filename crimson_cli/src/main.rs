mod cli_session;
mod command;
mod command_parser;

use crate::cli_session::CLISession;

fn main() {
    let mut session: CLISession = CLISession::new();
    session.run();
}