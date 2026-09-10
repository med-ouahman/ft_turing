

use std::{collections::{HashMap, HashSet}, print, println};

use json::{self, JsonValue, Parser};


// machine
#[derive(Debug)]
pub enum MachineError {
    InvalidInput,
    InvalidDescription,
    InvalidTransition,
    MissingField(String)
}

#[derive(Debug)]
pub(crate) struct Transition {
    pub(crate) read: char,
    pub(crate) to_state: String,
    pub(crate) write: char,
    pub(crate) action: String,
}

struct MachineDescriptor {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>
}

pub struct Machine {
    descriptor: MachineDescriptor
}

impl Machine {

pub(crate) fn initial_state(&self) -> String {
    self.descriptor.initial.clone()
}

pub(crate) fn blank_symbol(&self) -> char {
    self.descriptor.blank
}

pub(crate) fn is_final(&self, state: &str) -> bool {
    self.descriptor.finals.iter().any(|final_state| final_state == state)
}

pub(crate) fn transition(&self, state: &str, symbol: char) -> Option<&Transition> {
    self.descriptor.transitions.get(state)?.iter()
        .find(|transition| transition.read == symbol)
}
fn single_character(value: &str) -> Result<char, MachineError> {
    let mut characters = value.chars();
    let character = characters.next();
    match character {
        Some(character) if characters.next().is_none() => Ok(character),
        _ => Err(MachineError::InvalidDescription)
    }
}

fn required_string(object: &HashMap<String, JsonValue>, field: &str) -> Result<String, MachineError> {
    match object.get(field) {
        Some(JsonValue::String(value)) => Ok(value.clone()),
        None => Err(MachineError::MissingField(field.to_string())),
        _ => Err(MachineError::InvalidDescription)
    }
}

fn required_character(object: &HashMap<String, JsonValue>, field: &str) -> Result<char, MachineError> {
    let value = Self::required_string(object, field)?;
    Self::single_character(&value)
}

pub fn from_json(json_value: json::JsonValue) -> Result<Self, MachineError> {
    let object = match json_value {
        JsonValue::Object(object) => object,
        _ => return Err(MachineError::InvalidDescription)
    };

    let name = match object.get("name") {
        Some(JsonValue::String(name)) => name.clone(),
        _ => return Err(MachineError::InvalidDescription)
    };

    let alphabet_values = match object.get("alphabet") {
        Some(JsonValue::Array(values)) => values,
        _ => return Err(MachineError::InvalidDescription)
    };

    let mut alphabet = Vec::with_capacity(alphabet_values.len());
    for value in alphabet_values {
        let character = match value {
            JsonValue::String(value) => {
                let mut characters = value.chars();
                let character = characters.next();
                if character.is_none() || characters.next().is_some() {
                    return Err(MachineError::InvalidDescription);
                }
                match character {
                    Some(character) => character,
                    None => return Err(MachineError::InvalidDescription)
                }
            }
            _ => return Err(MachineError::InvalidDescription)
        };
        if !alphabet.contains(&character) {
            alphabet.push(character);
        } else {
            return Err(MachineError::InvalidDescription);
        }
    }

    let blank = match object.get("blank") {
        Some(JsonValue::String(value)) => Self::single_character(value)?,
        _ => return Err(MachineError::MissingField("blank".to_string()))
    };
    if !alphabet.contains(&blank) {
        return Err(MachineError::InvalidDescription);
    }

    let state_values = match object.get("states") {
        Some(JsonValue::Array(values)) => values,
        _ => return Err(MachineError::MissingField("states".to_string()))
    };
    let mut states = Vec::with_capacity(state_values.len());
    let mut state_names = HashSet::new();
    for value in state_values {
        let state = match value {
            JsonValue::String(state) if !state.is_empty() => state.clone(),
            _ => return Err(MachineError::InvalidDescription)
        };
        if !state_names.insert(state.clone()) {
            return Err(MachineError::InvalidDescription);
        }
        states.push(state);
    }

    let initial = match object.get("initial") {
        Some(JsonValue::String(initial)) if state_names.contains(initial) => initial.clone(),
        _ => return Err(MachineError::InvalidDescription)
    };

    let final_values = match object.get("finals") {
        Some(JsonValue::Array(values)) => values,
        _ => return Err(MachineError::MissingField("finals".to_string()))
    };
    let mut finals = Vec::with_capacity(final_values.len());
    let mut final_names = HashSet::new();
    for value in final_values {
        let final_state = match value {
            JsonValue::String(final_state) if state_names.contains(final_state) => final_state.clone(),
            _ => return Err(MachineError::InvalidDescription)
        };
        if !final_names.insert(final_state.clone()) {
            return Err(MachineError::InvalidDescription);
        }
        finals.push(final_state);
    }

    let transition_values = match object.get("transitions") {
        Some(JsonValue::Object(values)) => values,
        _ => return Err(MachineError::MissingField("transitions".to_string()))
    };
    let mut transitions = HashMap::new();
    for (state, values) in transition_values {
        if !state_names.contains(state) {
            return Err(MachineError::InvalidDescription);
        }
        let values = match values {
            JsonValue::Array(values) => values,
            _ => return Err(MachineError::InvalidDescription)
        };
        let mut state_transitions = Vec::with_capacity(values.len());
        let mut read_symbols = HashSet::new();
        for value in values {
            let transition = match value {
                JsonValue::Object(transition) => transition,
                _ => return Err(MachineError::InvalidDescription)
            };
            let read = Self::required_character(transition, "read")?;
            let to_state = Self::required_string(transition, "to_state")?;
            let write = Self::required_character(transition, "write")?;
            let action = Self::required_string(transition, "action")?;
            if !alphabet.contains(&read)
                || !alphabet.contains(&write)
                || !state_names.contains(&to_state)
                || (action != "LEFT" && action != "RIGHT")
                || !read_symbols.insert(read)
            {
                return Err(MachineError::InvalidDescription);
            }
            state_transitions.push(Transition { read, to_state, write, action });
        }
        transitions.insert(state.clone(), state_transitions);
    }

    Ok(Self { descriptor: MachineDescriptor { name, alphabet, blank, states, initial, finals, transitions } })
}

pub fn from_json_file(file: &str) -> Result<Self, MachineError> {
    let contents = std::fs::read_to_string(file).map_err(|_| MachineError::InvalidDescription)?;
    
    let mut parser = Parser::new(&contents).map_err(|_| MachineError::InvalidDescription)?;
    let json_value = parser.parse().map_err(|_| MachineError::InvalidDescription)?;
    Self::from_json(json_value)
}

pub fn validate_input(&self, input: &str) -> Result<(), MachineError> {
    if input.chars().all(|character| self.descriptor.alphabet.contains(&character)
        && character != self.descriptor.blank) {
        Ok(())
    } else {
        Err(MachineError::InvalidInput)
    }
}

pub(crate) fn display(&self) {
    println!("Machine: {}", self.descriptor.name);
    print!("Alphabet: [ ");
    for a in &self.descriptor.alphabet {
        print!("{a} ");
    }
    println!(" ]");
    print!("States: [ ");
    for state in &self.descriptor.states {
        print!("{state} ");
    }
    println!(" ]");
    println!("Initial: {}", self.descriptor.initial);
    print!("Finals: [ ");
    for state in &self.descriptor.finals {
        print!("{state} ");
    }
    println!(" ]");

}

}
