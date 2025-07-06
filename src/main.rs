mod script;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: 1 argument needed");
        return;
    }
    let path = &args[1];
    if !path.ends_with(".dice") {
        eprintln!("Error: file must end with \".dice\"");
        return;
    }
    let mut script = script::Script::load(path);

    script.run();
}
