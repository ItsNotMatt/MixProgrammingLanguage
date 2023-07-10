use core::time;

use crate::{ast::Expr, lexer::Token};

use super::Parser;

pub fn parse_if(parser: &mut Parser) {
    let expr = parser.get_expr();
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering if statement");
                parser.parse_tokens(Some(parser.nest));//so the if statement can know when its over
            }
            else {
                println!("\n----Skipping if block----");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

fn skip_block(parser: &mut Parser) {
    let mut nest: Option<usize> = None;
    while let Some(token) = parser.next_token() {
        match token {
            Token::OCurly => {
                if let Some(ref mut nst) = nest {
                    *nst += 1;
                }
                else {
                    nest = Some(1);
                }
            } 
            Token::CCurly => { 
                if let Some(ref mut nst) = nest {
                    *nst -= 1;
                }
                else {
                    panic!("Missing delimiter. Curr token: {:?}", token);
                }
            }
            Token::Eof => panic!("Hit Eof but delimiter is missing"),
            _ => {
                if nest == Some(0) {
                    panic!("Missing delimiter. Curr token: {:?}", token);
                }
                else {
                    println!("Skipping token: {:?}", token);
                }
            }
        }
        if nest == Some(0) {
            println!("----Skipped block----\n");
            break;
        }
    }
}

pub fn parse_while(parser: &mut Parser, expr: Option<Expr>) {//need to start copying at expression not at Ocurly
    println!("attempting to enter while loop");
    parser.consume_tokens = false;
    let expr = expr.unwrap_or_else(|| parser.get_expr());
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering while statement");
                parser.parse_tokens(Some(parser.nest));
                std::thread::sleep(time::Duration::from_millis(20));
                parser.position = 0;
                parse_while(parser, None);
            }
            else {
                parser.consume_tokens = true;
                //clear_copied_tokens(parser);
                println!("\n----Skipping while block----");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}



