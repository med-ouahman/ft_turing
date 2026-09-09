mod machine;
mod simulator;
mod tape;

use std::io::Read;

use crate::machine::Machine;
use crate::simulator::Simulator;
use crate::tape::Tape;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        println!("Usage: {} <filename> [input]", args[0]);
        return;
    }

    let filename = &args[1];
    
    let machine = Machine::from_json_file(filename).unwrap();
    
    let input = match args.get(2) {
        Some(input) => input.clone(),
        None => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            input.trim_end_matches(['\n', '\r']).to_string()
        }
    };

    machine.validate_input(&input).unwrap();

    let tape = Tape::new(&input, machine.blank_symbol());

    let mut simulator = Simulator::from_machine(machine, tape);
    
    simulator.run().unwrap();
}
