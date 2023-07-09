use std::{any::Any, cell::RefCell};

use crate::{lexer::Token, data_types::Function, ast::Expr};

use super::Parser;

pub fn parse_function(parser: &mut Parser, hash: u64) -> Option<Expr> {
    let mut args = Vec::new();
    match parser.next_token().unwrap() {
        Token::OParen => {
            if parser.peek_token().unwrap() == &Token::CParen {
                return call_native(parser, hash, args);
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

    return call_native(parser, hash, args);
}

fn call_native(parser: &mut Parser, hash: u64, args: Vec<Expr>) -> Option<Expr> {
    let func = parser.cache.get_fn_from_hash(hash);
    if let Some(f) = func.func.as_ref() {
        return (f)(args);
    }
    None
}
