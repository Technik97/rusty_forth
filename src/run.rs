use std::fs;
use std::io::{self, Write};

pub fn interpret(filename: &String) {
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    println!("{}", contents);
}

pub fn repl() {
    loop {
        print!("forth>");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("{}", input);
    }
}
