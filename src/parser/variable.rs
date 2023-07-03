use crate::{lexer::Token, ast::{Expr, ArithmeticOperator, Operator, BinExpr}, data_types::{Type, IntType, Variable}, parser::variable, runtime, evaluator::eval_bin_expr};

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
                    let expr = parser.get_expr();
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

fn change_val_by_expr(parser: &mut Parser, hash: u64, operator: Operator) {
    let expr = parser.get_expr();
    let var = parser.cache.get_var_from_hash(hash);
    let var_expr = var.to_expression();
    let bin_expr = Expr::BinExpr(BinExpr {
        left: Box::new(var_expr),
        op: operator,
        right: Box::new(expr), 
    });
    let value = eval_bin_expr(bin_expr);
    var.reassign_data_from_expr(value);
}

pub fn edit_var(parser: &mut Parser, hash: u64) {
    println!("Editing variable");

    match parser.next_token().unwrap() {
        Token::Operator(op) => {
            match op {
                Operator::Arithmetic(ArithmeticOperator::AddEq) => {
                    change_val_by_expr(parser, hash, Operator::Arithmetic(ArithmeticOperator::Add));
                }
                Operator::Arithmetic(ArithmeticOperator::SubEq) => {
                    change_val_by_expr(parser, hash, Operator::Arithmetic(ArithmeticOperator::Sub));
                }
                _ => panic!("Not valid token after var"),
            }
        }
        Token::Equal => {
            let expr = parser.get_expr();
            reassign_var(parser, hash, expr);
        }
        _ => {
            panic!("Not valid token after var");
        }
    }
}




