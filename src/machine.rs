
use std::collections::HashMap;

use json::{self, JsonValue, Parser, Token};

// machine
#[derive(Debug)]
pub enum MachineError {
    InvalidInput,
    InvalidDescription,
    MissingField(String)
}

enum MachineDescriptor {
    Name(String),

}

pub struct Machine {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: JsonValue
}

impl Machine {

pub fn from_json(json_value: json::JsonValue) -> Result<Self, MachineError> {
    let _ = json_value;
    Err(MachineError::InvalidInput)
}

pub fn from_json_file(file: &str) -> Result<Self, MachineError> {
    
    let mut parser = match Parser::new(file) {
        Ok(p) => p,
        Err(_) => return Err(MachineError::InvalidDescription)
    };

    let json_value = match parser.parse() {
        Ok(value) => value,
        Err(_) => return Err(MachineError::InvalidDescription)
    };

    let object = match json_value {
        JsonValue::Object(obj) => obj,
        _ => return Err(MachineError::InvalidDescription)
    };

    let name = match object.get("name") {
        Some(JsonValue::String(s)) => s,
        _ => return Err(MachineError::InvalidDescription)
    };

    let alphabet = match object.get("alphabet") {
        Some(JsonValue::Array(arr)) => arr,
        _ => return Err(MachineError::InvalidDescription)
    };

    let blank = object.get("blank").ok_or(MachineError::MissingField("blank".to_string()));
    let states = object.get("state").ok_or(MachineError::MissingField("states".to_string()));
    let initial = object.get("initial").ok_or(MachineError::MissingField("initial".to_string()));
    let finals = object.get("finals").ok_or(MachineError::MissingField("finals".to_string()));
    let transitions = object.get("transitions").ok_or(MachineError::MissingField("transitions".to_string()));


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
