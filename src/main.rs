#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        display_prompt_dollar_sign();

        let input = read_command();
        let command: Vec<&str> = input.split_whitespace().collect();
        let cmd = command[0];
        let args = &command[1..];

        match cmd {
            "exit" => break,
            "echo" => echo(args),
            _ => evaluate(cmd)
        }
    }
}

fn echo (args: &[&str]) {
    println!("{}", args.join(" "));
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

fn evaluate(cmd: &str) {
    println!("{}: command not found", cmd.trim());
}