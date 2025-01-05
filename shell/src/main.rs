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
        let tokens = tokenize(command);       //remove some in between whitespace also makes it vector of strings
        match tokens.as_slice() {
            // Handle exit command (optionally with a return code)
            ["exit", code] => {
                let exit_code = code.parse::<i32>().unwrap_or(0); // Default to 0 if parsing fails
                exit(exit_code);
            }
            // Handle echo command
            ["echo", ..] => {
                // Print everything after "echo"
                println!("{}", tokens[1..].join(" "));
            }
            // Handle type command
            ["type", ..] => {
                if tokens.len() > 1 {
                    let command_name = tokens[1];
                    if command_name == "exit" || command_name == "echo" || command_name == "type" {
                        println!("{} is a shell builtin", command_name);
                    } else {
                        println!("{}: not found", command_name);
                    }
                } 
            }
            _ => println!("{} : not found", tokens.join(" ")),
        }
    }
}
    

