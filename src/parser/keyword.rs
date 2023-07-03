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
                skip_if(parser);
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}

fn skip_if(parser: &mut Parser) {
    let mut nest = 0;
    while let Some(token) = parser.next_token() {
        match token {
            Token::OCurly => nest += 1,
            Token::CCurly => nest -= 1,
            Token::Eof => panic!("Hit Eof but delimiter is missing"),
            _ => {
                if nest == 0 {
                    panic!("Missing delimiter");
                }
                else {
                    println!("Skipping tokens");
                }
            }
        }
        if nest == 0 {
            break;
        }
    }
}

