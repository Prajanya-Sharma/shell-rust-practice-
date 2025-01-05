#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();                          //take input
        let mut input = String::new();                  //input can be mutable ofcourse
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();                      //no trailing white spaces
        let token = tokenize(command);       //remove some in between whitespace also makes it vector of strings
        match token.as_slice() {                              //cannot directly destructure a Vec<T> because it is a heap-allocated dynamic collection. Pattern matching works only on fixed-size collections like arrays or slices.
            ["exit", code] => exit(code.parse::<i32>().unwrap()),
            ["echo", ..] => println!("{}", token.join(" ")),
            _ => println!("{} : not found", token.join(" ")),
        }
    }
}
