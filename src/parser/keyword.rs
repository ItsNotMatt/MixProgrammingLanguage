use core::time;

use crate::{ast::{Expr, Key}, lexer::Token};

use super::Parser;

pub fn parse_if(parser: &mut Parser) {
    let expr = parser.get_expr();
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering if statement");
                parser.parse_tokens(Some(parser.nest));//so parse_tokens knows when it hits end of block 
            }
            else {
                println!("\n----Skipping if block----");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

//generic name incase we need it to parse something else other than else later
pub fn parse_block(parser: &mut Parser) {
    println!("Parsing else block");
    parser.parse_tokens(Some(parser.nest));//so parse_tokens knows when it hits end of block
}

pub fn skip_block(parser: &mut Parser) -> Vec<Token> {
    println!("Nest is: {}", parser.nest);
    if parser.nest == 0 { parser.consume_tokens = true;}
    else { parser.consume_tokens = false;}
    let mut nest: Option<usize> = None;
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = parser.next_token() {
        match token {
            Token::OCurly => {
                if let Some(ref mut nst) = nest { *nst += 1; }
                else { nest = Some(1); }
            } 
            Token::CCurly => { 
                if let Some(ref mut nst) = nest { *nst -= 1; }
                else { panic!("Missing delimiter. Curr token: {:?}", token); }
            }
            Token::Eof => panic!("Hit Eof but delimiter is missing"),
            _ => {
                if nest == Some(0) { panic!("Missing delimiter. Curr token: {:?}", token); }
                else { println!("Skipping token: {:?}", token); }
            }
        }
        tokens.push(token);
        if nest == Some(0) {
            println!("----Skipped block----\n");
            break;
        }
    }
    //else is only triggered if skip block happened meaning it must be the case
    if parser.peek_token().unwrap()  == &Token::Keyword(Key::Else) {
        parser.next_token().unwrap(); 
        parse_block(parser);
    }
    return tokens;
}

pub fn parse_while(parser: &mut Parser, expr: Option<Expr>) {//need to start copying at expression not at Ocurly
    parser.consume_tokens = false;
    parser.position = parser.read_position[parser.read_position.len() - 1];
    println!("Attempting to enter while loop at position: {}", parser.position);
    let expr = expr.unwrap_or_else(|| parser.get_expr());
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering while statement");
                let expr_bool = parser.parse_tokens(Some(parser.nest));
                parse_while(parser, expr_bool);
            }
            else {
                parser.read_position.remove(parser.read_position.len() - 1);
                println!("\n----Skipping while block----");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}



