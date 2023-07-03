use crate::{lexer::Token, ast::{Expr, ArithmeticOperator, Operator}, data_types::{Type, IntType, Variable}, parser::variable, runtime};

use super::Parser;

fn create_var(parser: &mut Parser, identifier: String, expr: Expr) {
    let data_type: Type = match expr {
            Expr::Number(n) => {
                Type::Int(IntType { value: n })
            }
            _ => {
                panic!("Unable to create var");
            }
        };
        let var = Variable::new(identifier, data_type); 
        parser.cache.add_var(var);
}

pub fn assign_var(parser: &mut Parser) {
    let token = parser.next_token().unwrap();
    match token {
        Token::Identifier(s) => {

            let token = parser.next_token().unwrap();
            match token {//should be = after var
                Token::Equal => {
                    let expr = get_expr(parser);
                    create_var(parser, s, expr);
                }
                _ => {
                    panic!("Operand not supported after declaration {}", s);
                }
            }
        }
        _ => {
            panic!("Illegal token after Let");
        }
    }
}

fn reassign_var(parser: &mut Parser, hash: u64, expr: Expr) {
    let var = parser.cache.get_var_from_hash(hash);
    match &mut var.data_type {
        Type::Int(i) => {
             match expr {
                 Expr::Number(n) => {
                     i.value = n;
                     println!("Reassigning {}, to {} ", var.name, i.value);
                 }
                 _ => {
                    panic!("Unsupported reassignment to var, cant reassign var to this type");
                 }
             }
        }
        _ => panic!("Unsupported reassignment to var, cant reassign var to this type"),
    }
}

pub fn edit_var(parser: &mut Parser, hash: u64) {
    println!("Editing variable");

    match parser.next_token().unwrap() {
        Token::Operator(op) => {
            match op {
                Operator::Arithmetic(ArithmeticOperator::AddEq) => {
                }
                _ => panic!("Not valid token after var"),
            }
        }
        Token::Equal => {
            let expr = get_expr(parser);
            reassign_var(parser, hash, expr);
        }
        _ => {
            panic!("Not valid token after var");
        }
    }
}

fn get_expr(parser: &mut Parser) -> Expr {
    if parser.tokens[1] != Token::Semi {
        let expr = parser.parse_bin_expr(None);
        return expr;
    }
    else {
        match parser.next_token().unwrap() {
            Token::Number(n) => return Expr::Number(n),
            Token::Identifier(s) => {
                let var = parser.cache.get_var_from_string(&s);
                return var.to_expression();
            }
            _ => panic!("Couldnt properly parse token for var"),
        }
    }
}


