use std::{any::Any, cell::RefCell, collections::HashMap, thread::panicking};

use crate::{lexer::Token, data_types::{Function, self, Type, CustomFunction, Variable}, ast::{Expr, Key}};

use super::{Parser, variable, keyword::{skip_block, save_block}};

//expr used as self for chaining. ex: input().to_int(). input() is str and passed to this fn
pub fn parse_function(parser: &mut Parser, hash: u64, expr: Option<Expr>, native: bool) -> Option<Expr> {
    let mut args = Vec::new();
    if let Some(exp) = expr {
        args.push(exp);
    }
    match parser.next_token().unwrap() {
        Token::OParen => {
            if parser.peek_token().unwrap() == &Token::CParen {
                parser.next_token().unwrap(); //to get  rid of cparen
                if native { return call_native(parser, hash, args); }
                else { return call_custom(parser, hash, args); }
            }
            loop {
                let expr = parser.get_expr();
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
    if native { return call_native(parser, hash, args); }
    else { return call_custom(parser, hash, args); }
}

fn call_native(parser: &mut Parser, hash: u64, args: Vec<Expr>) -> Option<Expr> {
    let func = parser.cache.get_fn_from_hash(hash);
    return (func.func)(args);
}

fn call_custom(parser: &mut Parser, hash: u64, args: Vec<Expr>) -> Option<Expr> {
    //check args
    parse_args(parser, hash, args);
    let func = parser.cache.get_custom_from_hash(hash);
    
    let return_position = parser.position;
    parser.position = func.body.start;
    parser.consume_tokens = false;
    let return_val = parser.parse_tokens(Some(parser.nest));
    let func = parser.cache.get_custom_from_hash(hash);
    //if return val != the key return_val expected then error
    match (&return_val, &func.return_val) {
        (None, None) => {}
        (Some(Expr::Number(n)), Some(Key::Int)) => { println!("Returning value: {}", n)}
        (Some(Expr::String(s)), Some(Key::String)) => { println!("Returning value: {}", s)}
        (Some(Expr::Bool(b)), Some(Key::Bool)) => { println!("Returning value: {}", b)}
        _ => panic!("Return value doesnt match expected return value")
    }

    parser.position = return_position;
    remove_temp_vars(parser, hash);

    return_val 
}

fn parse_args(parser: &mut Parser, hash: u64, mut args: Vec<Expr>) {
    let func = parser.cache.get_custom_from_hash(hash);
    let mut vars: Vec<Variable> = Vec::new();

    if args.len() != func.variables.len() {
        panic!("Function takes {} args but {} were passed in", args.len(), func.variables.len());
    } 
    else {
        //need to match arg type to temp var type and then create a var or set temp var datatype to
        //expr to be used in func
        for (_, value) in &mut func.variables {
            let arg = args.remove(0);
            match value.type_requirement  {
                Key::Int => {
                    match arg {
                        Expr::Number(n) => {
                            value.data_type = Some(Type::Int(n));
                            vars.push(value.convert_to_var());
                        } 
                        _ => panic!("Invalid arg type, expected int"),
                    }
                }
                Key::String => {
                    match arg {
                        Expr::String(s) => {
                            value.data_type = Some(Type::String(s));
                            vars.push(value.convert_to_var());
                        } 
                        _ => panic!("Invalid arg type, expected string"),
                    }
                }
                Key::Bool => {
                    match arg {
                        Expr::Bool(b) => {
                            value.data_type = Some(Type::Bool(b));
                            vars.push(value.convert_to_var());
                        } 
                        _ => panic!("Invalid arg type, expected bool"),
                    }
                }
                _ => panic!("Key cant be a type requirement"),
            };
        }
    }
    for var in vars {
        parser.cache.add_var(var);
    }

}

fn remove_temp_vars(parser: &mut Parser, hash: u64) {
    let func = parser.cache.get_custom_from_hash(hash);
    let mut vars: Vec<u64> = Vec::new();
    for hash in func.variables.keys() {
        vars.push(*hash);
    }
    parser.cache.remove_temps(vars);
}

pub fn declare_custom(parser: &mut Parser) {
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
            let mut return_val = None;
            if parser.peek_token().unwrap()  == &Token::Colon {
                parser.next_token().unwrap();
                if let Token::Keyword(key) = parser.next_token().unwrap() {
                    return_val = Some(key);
                }
            }
            let range = save_block(parser);
            //have to check for return value first
            
            let func = data_types::CustomFunction::new(f, temp_vars, range, return_val);
            parser.cache.add_custom(func);
        }
        _ => panic!("Token after fn is illegal, expeceted identifier"),
    }
}

//need to pass var to function instead of expr like in parse fn chain
pub fn parse_var_chain(parser: &mut Parser, hash: u64) -> Expr {
    let var = parser.cache.get_var_from_hash(hash);
    todo!();
}

//need to parse function and then get the return value if it has one and then use that in the chain
pub fn parse_fn_chain(parser: &mut Parser, mut expr: Expr) -> Expr {
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
//not set up for custom funcs yet
fn next_fn(parser: &mut Parser, name: &String, expr: Expr) -> Expr {
    if let Some(hash) = parser.cache.get_fn_hash(name) {
        return parse_function(parser, hash, Some(expr), true).unwrap();
    }
    else { panic!("Cant find function by that name");}
}

