#![allow(dead_code)]
use std::{fs::File, io::Read};

use cli::CLI;

mod lexer;
mod ast;
mod parser;
mod error;
mod evaluator;
mod runtime;
mod data_types;
mod core;
mod cli;
mod lib;

fn main() {
    //read from file
    let dir = std::env::current_dir().unwrap();
    let file_path = dir.join("src/main.mx");
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //get args  arg should be -- -t     -t to just tokenize, no args means run code
    if cli::get_args() == CLI::Tokenize {
        let tokens = tokenize(contents);
        for token in tokens {
            println!("Token: {:?}", token);
        }
        return
    }

    let tokens = tokenize(contents);

    //create cache and import io into cache
    let cache = runtime::cache::Cache::new();

    //create parser and give values to cache from parser
    let mut parser = parser::Parser::new(tokens, cache);
    parser.parse_tokens(None);

}

fn tokenize(contents: String) -> Vec<lexer::Token> {
    //turn file into chars to be tokenized
    let src: Vec<char> = contents.chars().collect();
    let mut lexer = lexer::Lexer::new(src);
    let tokens = lexer.tokenize();
    return tokens;
}


