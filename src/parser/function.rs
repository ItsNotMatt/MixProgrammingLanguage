use std::{any::Any, cell::RefCell};

use crate::{lexer::Token, data_types::Function, ast::Expr};

use super::Parser;

pub fn parse_function(parser: &mut Parser, hash: u64) {
    let mut args = Vec::new();
    match parser.next_token().unwrap() {
        Token::OParen => {
            if parser.peek_token().unwrap() == &Token::CParen {
                call_native(parser, hash);
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

    if args.is_empty() { parser.cache.args = None; }
    else { parser.cache.args = Some(args); }
    call_native(parser, hash);
}

fn call_native(parser: &mut Parser, hash: u64) {
    if let Some(args) = parser.cache.args.clone() {
        let func = parser.cache.get_fn_from_hash(hash);
        if let Some(f) = func.func.as_ref() {
            (f)(args);
        }
    }
    else {
        let func = parser.cache.get_fn_from_hash(hash);
        if let Some(f) = func.func.as_ref() {
            let args: Vec<Expr> = Vec::new();
            (f)(args);
        }
    }
    parser.cache.args = None;
}

fn call_native_as_expression(parser: &mut Parser, hash: u64) {
    if let Some(args) = parser.cache.args.clone() {
        let func = parser.cache.get_fn_from_hash(hash);
        if let Some(f) = func.func.as_ref() {
            (f)(args);
        }
    }
    else {
        let func = parser.cache.get_fn_from_hash(hash);
        if let Some(f) = func.func.as_ref() {
            let args: Vec<Expr> = Vec::new();
            (f)(args);
        }
    }
    parser.cache.args = None;
}

