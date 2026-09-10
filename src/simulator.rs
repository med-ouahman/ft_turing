
use std::println;

use crate::machine::{Machine, MachineError};
use crate::tape::Tape;

pub struct Simulator {
    machine: Machine,
    tape: Tape,
    state: String,
}

impl Simulator {
    pub fn new(machine: Machine, tape: Tape) -> Self {
    let state = machine.initial_state();
    Self { machine, tape, state }
}

pub fn from_machine(machine: Machine, tape: Tape) -> Self {
    Self::new(machine, tape)
}

pub fn run(&mut self) -> Result<(), MachineError> {

    self.machine.display();
    while !self.machine.is_final(&self.state) {
        
        let state = self.state.clone();
        let transition = match self.machine.transition(&self.state, self.tape.read()) {
            Some(transition) => transition,
            None => return Err(MachineError::InvalidTransition),
        };
        
        
        let read_symbol = self.tape.read();
        let next_state = transition.to_state.clone();
        let action = transition.action.clone();
        let write_symbol = transition.write;
        
        self.tape.display();

        println!("({state}, {read_symbol}) -> ({next_state}, {write_symbol}, {action})");

        self.tape.write(transition.write);

        if action == "LEFT" {
            self.tape.move_left();
        } else {
            self.tape.move_right();
        }
        self.state = next_state;
    }

    Ok(())
}
}