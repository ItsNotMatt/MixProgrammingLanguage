use std::any::Any;

use crate::{lexer::Token, data_types::Function, ast::Expr};

use super::Parser;

pub fn parse_function(parser: &mut Parser, hash: u64) {
    let mut args = Vec::new();
    match parser.next_token().unwrap() {
        Token::OParen => {
            loop {
                let expr = parser.get_expr();
                println!("adding expr {} to args", expr);
                args.push(expr);
                match parser.next_token().unwrap() {
                    Token::CParen => break,
                    Token::Comma => continue,
                    _ => panic!("Not valid token after arg"),
                }
            }
        }
        _ => panic!("Token after function isnt valid"),
    }

    call_native(parser, hash, args);
}

fn call_native(parser: &mut Parser, hash: u64, args: Vec<Expr>) {
    let func = parser.cache.get_fn_from_hash(hash);
    let args: Box<dyn Any> = Box::new(args);
    if let Some(f) = func.func.as_ref() {
        (f)(args);
    }
}


