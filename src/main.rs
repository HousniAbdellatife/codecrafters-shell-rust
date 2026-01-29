#[allow(unused_imports)]
use std::io::{self, Write};
use std::io::Read;

fn main() {
    loop {
        display_prompt_dollar_sign();
        let command  = read_command();
        if command.eq("exit") { break }
        evaluate(command);
    }
}

fn display_prompt_dollar_sign() {
    print!("$ ");
    io::stdout().flush().unwrap()
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Failed");

    command.trim().to_string()
}

fn evaluate(command: String) {
    println!("{}: command not found", command.trim());
}