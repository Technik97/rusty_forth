use std::env;

use rusty_forth::run::{interpret, repl};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && &args[1] == "-f" {
        let filename = &args[2];
        interpret(filename);
    } else {
        repl();
    }
}
