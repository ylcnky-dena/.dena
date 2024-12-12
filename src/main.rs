use std::{ env, fs, process::exit };
use std::io::{ self, BufRead, Write };

mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod environment;

use interpreter::Interpreter;
use crate::scanner::*;
use crate::parser::*;
use crate::stmt::{ Stmt::* };

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Err(msg) => {
            return Err(msg.to_string());
        }
        Ok(contents) => {
            return run(&mut interpreter, &contents);
        }
    }
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let smtms = parser.parse()?;
    interpreter.interpret(smtms)?;

    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        let mut buffer = String::new();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                return Err("Could not flush stdout".to_string());
            }
        }

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(_) => {
                return Err("Couldn't read line".to_string());
            }
        }
        println!("ECHO: {}", buffer);
        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: dena [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    }
}
