#[derive(Clone)]
pub enum CommandResult {
    Signal(Signal),
    Message(String),
    None,
}

#[derive(Clone)]
pub enum Signal {
    Quit,
    InitCPU,
    InitMemory,
    InitAll,
    CPUStep(isize),
    WriteMemory(u16, u8),
    FileOpen(String),
    FileRun,
}