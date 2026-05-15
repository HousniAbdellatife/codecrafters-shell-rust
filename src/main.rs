#[allow(unused_imports)]
use std::io::{self, Write};


const COMMANDS: [&str; 3] = ["echo", "type", "exit"];

fn main() {
    loop {
        display_prompt_dollar_sign();

        let input = read_command();
        let command: Vec<String> = shell_words::split(&input).unwrap();
        let cmd = command[0].as_str();
        let args = &command[1..].iter().map(|arg| arg.as_str()).collect::<Vec<&str>>();

        match cmd {
            "exit" => break,
            "echo" => echo(args),
            "type" => find_type(args[0]),
            _ => evaluate(cmd)
        }
    }
}

fn find_type(p0: &str) {
    if COMMANDS.contains(&p0) {
        println!("{} is a shell builtin", p0);
    }else {
        println!("{}: not found", p0);
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