use m2_rust_interpreter::{convert, lexer::tokenize, parser::parse};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <m2-source-file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let input = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("failed to read {}: {}", filename, err);
            process::exit(1);
        }
    };

    let tokens = match tokenize(&input, filename) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("lex error: {}", err);
            process::exit(1);
        }
    };

    let parse_tree = match parse(&tokens) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("parse error: {}", err);
            process::exit(1);
        }
    };

    let code = convert(parse_tree);
    println!("compiled code: {:#?}", code);
}
