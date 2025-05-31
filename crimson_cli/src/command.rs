#[derive(Clone)]
pub enum CommandResult {
    Signal(Signal),
    Message(String),
    None,
}

#[derive(Clone, Copy)]
pub enum Signal {
    Quit,
    CPUStep(isize),
    None,
}