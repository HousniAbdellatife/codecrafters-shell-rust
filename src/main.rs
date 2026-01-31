#[allow(unused_imports)]
use std::io::{self, Write};


const COMMANDS: [&str; 1] = [""];

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
            "type" => find_type(cmd),
            _ => evaluate(cmd)
        }
    }
}

fn find_type(p0: &str) {
    if COMMANDS.contains(&p0) {
        println!("{} is a shell builtin", p0);
    }else {
        println!("invalid_command: not found");
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