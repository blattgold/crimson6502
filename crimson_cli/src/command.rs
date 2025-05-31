#[derive(Clone)]
pub enum CommandResult {
    Signal(Signal),
    Message(String),
    None,
}

#[derive(Clone, Copy)]
pub enum Signal {
    Quit,
    InitCPU,
    InitMemory,
    CPUStep(isize),
}