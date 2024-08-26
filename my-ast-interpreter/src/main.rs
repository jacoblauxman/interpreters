use my_ast_interpreter::{Interpreter, Parser, Scanner};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize | parse | evaluate <filename>", args[0]);

        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => tokenize(file_contents),
        "parse" => parse(file_contents),
        "evaluate" => evaluate(file_contents),
        "run" => run(file_contents),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(file_contents: String) {
    let scanner = Scanner::new(file_contents);
    let (tokens, errors) = scanner.scan_tokens();

    for error in &errors {
        eprintln!("{}", error)
    }

    for token in tokens {
        println!("{}", token)
    }

    if !errors.is_empty() {
        process::exit(65)
    }
}

fn parse(file_contents: String) {
    let scanner = Scanner::new(file_contents);
    let (tokens, errors) = scanner.scan_tokens();

    for error in &errors {
        eprintln!("{}", error)
    }

    if !errors.is_empty() {
        process::exit(65)
    }

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(statements) => statements
            .iter()
            .for_each(|statement| println!("{statement}")),
        Err(parse_err) => {
            eprintln!("{}", parse_err);
            process::exit(65);
        }
    }
}

fn evaluate(file_contents: String) {
    let scanner = Scanner::new(file_contents);
    let (tokens, errors) = scanner.scan_tokens();

    for error in &errors {
        eprintln!("{}", error)
    }

    if !errors.is_empty() {
        process::exit(65)
    }

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(statements) {
                Ok(_) => (),
                Err(runtime_err) => {
                    eprintln!("{}", runtime_err);
                    process::exit(70);
                }
            }
        }
        Err(parse_err) => {
            eprintln!("{}", parse_err);
            process::exit(65);
        }
    }
}

fn run(file_contents: String) {
    let scanner = Scanner::new(file_contents);
    let (tokens, errors) = scanner.scan_tokens();

    for error in &errors {
        eprintln!("{}", error)
    }

    if !errors.is_empty() {
        process::exit(65)
    }

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            interpreter
                .set_status("run")
                .expect("should set interpreter status::run");
            match interpreter.interpret(statements) {
                Ok(_) => (),
                Err(runtime_err) => {
                    eprintln!("{}", runtime_err);
                    process::exit(70);
                }
            }
        }
        Err(parse_err) => {
            eprintln!("{}", parse_err);
            process::exit(65);
        }
    }
}
