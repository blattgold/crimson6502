#[derive(Clone, Copy)]
pub enum CommandResult {
    SignalQuit,
    None,
}

#[derive(Clone)]
pub enum CommandArgument {
    String(String),
    Number(isize),
}

#[derive(Clone, Copy)]
pub enum GrammarToken {
    Command,
    String,
    Num,
}

#[derive(Clone)]
pub struct Command {
    pub arg_count: usize,
    pub given_args: Vec<CommandArgument>,
    pub repr_str: String,
    pub grammar_def: Vec<GrammarToken>,
    pub execute: fn() -> CommandResult,
}

impl Command {
    pub fn new(
        arg_count: usize, 
        given_args: Vec<CommandArgument>,
        repr_str: String, 
        grammar_def: Vec<GrammarToken>, 
        execute: fn() -> CommandResult
    ) -> Command {
        Command {
            arg_count: arg_count,
            given_args: 
            repr_str: repr_str,
            grammar_def: grammar_def,
            execute: execute,
        }
    }
}
