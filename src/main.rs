#![allow(dead_code)]

use std::{fs::File, io::Read};

mod lexer;
mod ast;
mod parser;
mod error;
mod evaluator;
mod runtime;
mod data_types;

fn main() {
    let dir = std::env::current_dir().unwrap();
    let file_path = dir.join("src/mix.mx");
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //let source_code = String::from("let x = 10 + 2; let y = x + 3;");
    let src: Vec<char> = contents.chars().collect();
    let mut lexer = lexer::Lexer::new(src);
    let tokens = lexer.tokenize();

    let mut cache = runtime::cache::Cache::new();
    let mut parser = parser::Parser::new(tokens, cache);
    parser.parse_tokens();

}
