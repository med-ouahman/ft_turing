
use json::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let input = std::fs::read_to_string(filename).unwrap();

    match parse(&input) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(error) => println!("{:?}", error),
    }
}



