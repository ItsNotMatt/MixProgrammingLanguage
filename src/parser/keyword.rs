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
                println!("statement is false, skipping if statement");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

fn skip_block(parser: &mut Parser) {
    let mut nest = 0;
    while let Some(token) = parser.next_token() {
        match token {
            Token::OCurly => nest += 1,
            Token::CCurly => nest -= 1,
            Token::Eof => panic!("Hit Eof but delimiter is missing"),
            _ => {
                if nest == 0 {
                    panic!("Missing delimiter. Curr token: {:?}", token);
                }
                else {
                    println!("Skipping token: {:?}", token);
                }
            }
        }
        if nest == 0 {
            break;
        }
    }
}

pub fn parse_while(parser: &mut Parser) {//need to start copying at expression not at Ocurly
    println!("attempting to enter while loop");
    parser.consume_tokens = false;
    let expr = parser.get_expr();
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering while statement");
                parser.parse_tokens(Some(parser.nest));
                std::thread::sleep(time::Duration::from_millis(20));
                parser.read_position = parser.position;
                parser.position = 0;
                parse_while(parser);
            }
            else {
                parser.consume_tokens = true;
                clear_copied_tokens(parser);
                println!("statement is false, skipping while statement");
                skip_block(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

fn clear_copied_tokens(parser: &mut Parser) {
    println!("---Clearing tokens up to {:?}, pos: {}", parser.tokens[parser.read_position], parser.read_position);
    parser.tokens.drain(0..=parser.read_position); 
}



