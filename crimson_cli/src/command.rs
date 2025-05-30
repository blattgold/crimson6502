pub trait Command {
    const ARG_COUNT: usize;
    const STR: &'static str;
}

pub trait CommandVoid: Command {
    fn execute(&self);
}

pub trait CommandString: Command {
    fn execute(&self) -> String;
}
