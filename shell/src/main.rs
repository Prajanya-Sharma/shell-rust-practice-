#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn handle_commands(tokens: &[&str],path_env: &str){
    let shellcmds = tokens;
    let builtins = []
    match shellcmds {
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
                }#[allow(unused_imports)]
                use std::io::{self, Write};
                
                fn handle_command(input: &str, path_env: &str) -> bool { //takes path and input aas input
                    let parts: Vec<&str> = input.split_whitespace().collect(); // Just reference as input is also a reference
                    let builincmds = ["exit", "type", "echo"];
                
                    match parts[0] {
                        "exit" => {
                            if parts.len() > 1 && parts[1] == "0" {
                                return true; // Exit command successful
                            } else {
                                println!("{}: command not found", input);
                            }
                        }
                        "echo" => {
                            println!("{}", &input[5..]); // Print the input after "echo "
                        }
                        "type" => {
                            if parts.len() != 2 {
                                println!("type: expected 1 argument, got {}", parts.len() - 1); // Check for the number of arguments
                            } else if builincmds.contains(&parts[1]) {
                                println!("{} is a shell builtin", parts[1]); // Print for built-in commands
                            } else {
                                let split = &mut path_env.split(':'); //env split to &str
                                if let Some(path) = split.find(|path| {
                                    std::fs::metadata(format!("{}/{}", path, parts[1])).is_ok() //less goo path formatted
                                }) {
                                    // Corrected format string
                                    println!("{} is {}/{}", parts[1], path, parts[1]);
                                } else {
                                    println!("{}: not found", parts[1]); // Command not found in PATH
                                }
                            }
                        }
                        _ => {
                            println!("{}: command not found", input); // Default case for unknown commands
                        }
                    }
                    false
                }
                


fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn main() {


    loop {
        //trying options to handle can use unwrap too
        let path_env = match std::env::var("PATH") {
            Ok(path) => path, // Successfully retrieved PATH cheers
            Err(_) => {
                eprintln!("Error: PATH environment variable is not set!");
                return; // Exit the program gracefully
            }
        };
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();                          //take input
        let mut input = String::new();                  //input can be mutable ofcourse
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();                      //no trailing white spaces
        let tokens = tokenize(command); 
        
       handle_commands(&tokens, &path_env);
    }
}
    

