use crate::command::CommandResult;
use crate::command::Signal;

const ERR_ARG_COUNT_STR: &'static str = "Invalid number of arguments received for command: ";
const ERR_INVALID_ARG_STR: &'static str = "Invalid argument received: ";
const ERR_UNKNOWN_STR: &'static str = "Unknown command: ";

pub struct CommandParser {}

impl CommandParser {
    pub fn parse(input_slice: &[String]) -> CommandResult {
        if input_slice.is_empty() {
            return CommandResult::None;
        }

        match input_slice[0].as_str() {
            "quit" | "exit" => CommandResult::Signal(Signal::Quit),
            //----------------------------------------------------------------------------------------------
            "init" if input_slice.len() == 2 => {
                match input_slice[1].as_str() {
                    "cpu" => CommandResult::Signal(Signal::InitCPU),
                    "memory" | "mem" => CommandResult::Signal(Signal::InitMemory),
                    s => CommandResult::Message(String::from(ERR_INVALID_ARG_STR) + s)
                }
            } 
            "init" => CommandResult::Signal(Signal::InitAll),
            //----------------------------------------------------------------------------------------------
            "step" if input_slice.len() == 2 => {
                if let Ok(num) = input_slice[1].parse::<isize>() {
                    CommandResult::Signal(Signal::CPUStep(num))
                } else {
                    CommandResult::Message(String::from(ERR_INVALID_ARG_STR) + &input_slice[1])
                }
            },
            "step" if input_slice.len() == 1 => CommandResult::Signal(Signal::CPUStep(1)),
            "step" => CommandResult::Message(String::from(ERR_ARG_COUNT_STR) + "step"),
            //----------------------------------------------------------------------------------------------
            "write" if input_slice.len() == 3 => {
                if let (Ok(addr), Ok(val)) = (input_slice[1].parse::<isize>(), input_slice[2].parse::<isize>()) {
                    CommandResult::Signal(Signal::WriteMemory(addr as u16, val as u8))
                } else {
                    CommandResult::Message(String::from(ERR_INVALID_ARG_STR) + "write")
                }
            },
            "write" => CommandResult::Message(String::from(ERR_ARG_COUNT_STR) + "write"),
            //----------------------------------------------------------------------------------------------
            s => CommandResult::Message(String::from(ERR_UNKNOWN_STR) + s),
        }
    }
}