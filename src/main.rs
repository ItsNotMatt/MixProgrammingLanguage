#![allow(dead_code)]

use std::{fs::File, io::Read};

mod lexer;
mod ast;
mod parser;
mod error;
mod evaluator;
mod runtime;
mod data_types;
mod core;

fn main() {
    //read from file
    let dir = std::env::current_dir().unwrap();
    let file_path = dir.join("src/mix.mx");
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //turn file into chars to be tokenized
    let src: Vec<char> = contents.chars().collect();
    let mut lexer = lexer::Lexer::new(src);
    let tokens = lexer.tokenize();

    //create cache and import io into cache
    let mut cache = runtime::cache::Cache::new();
    core::import_io(&mut cache);

    //create parser and give values to cache from parser
    let mut parser = parser::Parser::new(tokens, cache);
    parser.parse_tokens();

}
