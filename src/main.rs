pub mod dice;
mod script;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: 1 argument needed");
        return;
    }
    let mut script = script::Script::load(&args[1]);

    script.run();
}
