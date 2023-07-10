use std::{any::Any, cell::RefCell};

use crate::{lexer::Token, data_types::Function, ast::Expr};

use super::Parser;

//expr used as self for chaining. ex: input().to_int(). input() is str and passed to this fn
pub fn parse_function(parser: &mut Parser, hash: u64, expr: Option<Expr>) -> Option<Expr> {
    let mut args = Vec::new();
    if let Some(exp) = expr {
        args.push(exp);
    }
    match parser.next_token().unwrap() {
        Token::OParen => {
            if parser.peek_token().unwrap() == &Token::CParen {
                parser.next_token().unwrap(); //to get  rid of cparen
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



pub fn parse_var_chain(parser: &mut Parser, hash: u64) -> Expr {
    println!("Parsing chain after var");
    let var = parser.cache.get_var_from_hash(hash);
    todo!();
}

//need to parse function and then get the return value if it has one and then use that in the chain
pub fn parse_fn_chain(parser: &mut Parser, mut expr: Expr) -> Expr {
    println!("Parsing chain after fn");
    match parser.next_token().unwrap() {
        Token::Identifier(s) => expr = next_fn(parser, &s, expr),
        _ => panic!("Token after . is illegal"),
    }
    if parser.peek_token().unwrap() == &Token::Dot {
        parser.next_token().unwrap(); //to get rid of dot
        expr = parse_fn_chain(parser, expr);
    }
    expr
}

//expr is prev functions expression return value
fn next_fn(parser: &mut Parser, name: &String, expr: Expr) -> Expr {
    if let Some(hash) = parser.cache.get_fn_hash(name) {
        return parse_function(parser, hash, Some(expr)).unwrap();
    }
    else { panic!("Cant find function by that name");}
}

