use std::{any::Any, cell::RefCell, collections::HashMap};

use crate::{lexer::Token, data_types::{Function, self}, ast::{Expr, Key}};

use super::{Parser, variable, keyword::skip_block};

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
    return (func.func)(args);
}

pub fn declaration(parser: &mut Parser) {
    match parser.next_token().unwrap() {
        Token::Identifier(f) => {
            let mut temp_vars: HashMap<String, Key> = HashMap::new();
            match parser.next_token().unwrap() {
                Token::OParen => {
                    loop {
                        match parser.next_token().unwrap() {
                            Token::Identifier(s) => {
                                match parser.next_token().unwrap() {
                                    Token::Colon => match parser.next_token().unwrap() {
                                        Token::Keyword(k) => temp_vars.insert(s, k),
                                        _ => panic!("Token illegal, expected keyword"),
                                    }
                                    _ => panic!("Token illegal, expected colon"),
                                };
                            }
                            Token::Comma => continue,
                            Token::CParen => break,
                            _ => panic!("Token illegal, expected var declaration"),
                        };
                    }
                }
                _ => panic!("Token after function name illegal, expected paren"),
            }
            let temp_vars = variable::make_temp_vars(temp_vars);
            let tokens = skip_block(parser);
            let func = data_types::CustomFunction::new(f, temp_vars, tokens);
            parser.cache.add_custom_fn(func);
        }
        _ => panic!("Token after fn is illegal, expeceted identifier"),
    }
}

//need to pass var to function instead of expr like in parse fn chain
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

