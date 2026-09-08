
use json::{self};

// machine
#[derive(Debug)]
enum MachineError {
    InvalidInput
}

pub struct Machine {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    finals: Vec<String>
}

impl Machine {

fn
pub fn from_json(json_value: json::JsonValue) -> Result<Self, MachineError> {
    let _ = json_value;
    Err(MachineError::InvalidInput)
}

pub fn from_json_file(file: &str) -> Result<Self, MachineError> {
    let _ = file;
    Err(MachineError::InvalidInput)
}


fn validate_input(&self, input: &str) -> Result<(), MachineError> {
    let _ = input;
    Err(MachineError::InvalidInput)
}

pub fn run(&mut self, input: &str) {
    let _ = self.validate_input(input);

}

}
