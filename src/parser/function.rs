use std::any::Any;

use crate::{lexer::Token, data_types::Function, ast::Expr};

use super::Parser;

pub fn parse_function(parser: &mut Parser, hash: u64) {
    let mut args = Vec::new();
    match parser.next_token().unwrap() {
        Token::OParen => {
            if parser.peek_token().unwrap() == &Token::CParen {
                call_native(parser, hash, None);
                return;
            }
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

    call_native(parser, hash, Some(args));
}

fn call_native(parser: &mut Parser, hash: u64, args: Option<Vec<Expr>>) {
    let func = parser.cache.get_fn_from_hash(hash);
    if let Some(f) = func.func.as_ref() {
        if let Some(args) = args {
            let args: Box<dyn Any> = Box::new(args);
            (f)(args);
        }
        else {
            println!("No function arguments passed");
            let args: Box<dyn Any> = Box::new(-69420);
            (f)(args);
        }
    }
}


