use core::time;
use std::ops::Range;

use crate::{ast::{Expr, Key}, lexer::Token};

use super::Parser;

pub fn parse_if(parser: &mut Parser) {
    let expr = parser.get_expr();
    match expr {
        Expr::Bool(b) => {
            if b {
                parser.parse_tokens(Some(parser.nest));//so parse_tokens knows when it hits end of block 
            }
            else {
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

//generic name incase we need it to parse something else other than else later
pub fn parse_block(parser: &mut Parser) {
    parser.parse_tokens(Some(parser.nest));//so parse_tokens knows when it hits end of block
}

pub fn parse_while(parser: &mut Parser, expr: Option<Expr>) {
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

pub fn skip_block(parser: &mut Parser) { //move to parser?
    println!("Nest is: {}", parser.nest);
    if parser.nest == 0 { parser.consume_tokens = true;}
    else { parser.consume_tokens = false;}
    let mut nest: Option<usize> = None;
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
            }
        }
        if nest == Some(0) {
            break;
        }
    }
    //else is only triggered if skip block happened meaning it must be the case
    if parser.peek_token().unwrap()  == &Token::Keyword(Key::Else) {
        parser.next_token().unwrap(); 
        parse_block(parser);
    }
}

<<<<<<< HEAD
pub fn save_block(parser: &mut Parser) -> Range<usize> { 
    parser.consume_tokens = false;
    let start = parser.position;
    let mut nest = 0;
    while let Some(token) = parser.next_token() {
        match token {
            Token::OCurly => nest += 1,
            Token::CCurly => nest -= 1, 
            Token::Eof => panic!("Hit Eof but delimiter is missing"),
            _ => {
                if nest == 0 { panic!("Missing delimiter. Curr token: {:?}", token); }
                else { println!("saving token: {:?}", token); }
=======
pub fn parse_while(parser: &mut Parser, expr: Option<Expr>) {//need to start copying at expression not at Ocurly
    parser.consume_tokens = false;
    let expr = expr.unwrap_or_else(|| parser.get_expr());
    match expr {
        Expr::Bool(b) => {
            if b {
                let expr = parser.parse_tokens(Some(parser.nest));
                //oh shit?
                std::thread::sleep(time::Duration::from_millis(20));
                parser.position = 0;
                parse_while(parser, expr);
            }
            else {
                parser.consume_tokens = true;
                //clear_copied_tokens(parser);
                skip_block(parser);
>>>>>>> d8a3686dcaa966dab86f5d867e3a50bb5d0b99ec
            }
        }
        if nest == 0 {
            println!("----Saved block----\n");
            let range = start..parser.position;
            parser.consume_tokens = true;
            return range;
        }
    }
    panic!("Invalid function encapsulation");
}
