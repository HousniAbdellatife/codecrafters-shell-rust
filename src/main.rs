#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        display_prompt_dollar_sign();
        let command  = read_command();
        if command.eq("exit") { break }

        match command.split_ascii_whitespace().next().unwrap() {
            "exit" => break,
            "echo" => echo(&command),
            _ => evaluate(command)
        }
    }
}

fn echo (cmd: &String) {
    let arg: String = cmd.split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");

    println!("{}", arg);
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