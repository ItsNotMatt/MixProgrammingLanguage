use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::{lexer::Token, ast::{Expr, ArithmeticOperator, Operator, BinExpr, Key}, data_types::{Type, Variable, TempVar, self}, parser::variable, runtime, evaluator::eval_bin_expr, lib};

use super::Parser;

pub fn make_temp_vars(vars: HashMap<String, Key>) -> HashMap<u64, TempVar> {
    let mut temps: HashMap<u64, TempVar> = HashMap::new();
    for (name, value) in vars.into_iter() {
        //have to turn string and key into a var with a default value
        let hash = lib::get_hash(&name);
        
        match value {
            Key::Int => {
                let var = data_types::TempVar::new(name, value);
                temps.insert(hash, var);
            }
            Key::String => {
                let var = data_types::TempVar::new(name, value);
                temps.insert(hash, var);
            }
            Key::Bool => {
                let var = data_types::TempVar::new(name, value);
                temps.insert(hash, var);
            }
            _ => panic!("Cant make paramater variable with this keyword"),
        }
    }
    return temps;
}

fn create_var(parser: &mut Parser, identifier: String, expr: Expr, mutable: bool) {
    let data_type: Type = match expr {
            Expr::Number(n) => {
                Type::Int(n)
            }
            Expr::String(s) => {
                Type::String(s)
            }
            Expr::Bool(b) => {
                Type::Bool(b)
            }
            Expr::Array(a) => {
                Type::Array(a)
            }
            _ => {
                panic!("Unable to create var of this type");
            }
        };
        let var = Variable::new(identifier, data_type, mutable); 
        parser.cache.add_var(var);
}

pub fn assign_var(parser: &mut Parser, mutable: bool) {
    let token = parser.next_token().unwrap();
    match token {
        Token::Identifier(s) => {
            let token = parser.next_token().unwrap();
            match token {//should be = after var
                Token::Equal => {
                    let expr = parser.get_expr();
                    create_var(parser, s, expr, mutable);
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

//when var set = to something
fn reassign_var(parser: &mut Parser, hash: u64, expr: Expr) {
    let var = parser.cache.get_var_from_hash(hash);
    if !var.mutable {
        panic!("Cant edit value of this variable");
    }
    match &mut var.data_type {
        Type::Int(i) => {
             match expr {
                 Expr::Number(n) => {
<<<<<<< HEAD
                     *i = n;
=======
                     i.value = n;
>>>>>>> d8a3686dcaa966dab86f5d867e3a50bb5d0b99ec
                 }
                 _ => panic!("Unsupported reassignment to var, cant reassign var to this type"),
             }
        }
        Type::Bool(b) => {
            match expr {
                Expr::Bool(bo) => {
                    *b = bo;
                }
                _ => panic!("Unsupported reassignment to var, cant reassign var to this type"),
            }
        }
        Type::String(s) => {
            match expr {
                Expr::String(str) => {
                    *s = str;
                }
                _ => panic!("Unsupported reassignment to var, cant reassign var to this type"),
            }
        }
        _ => panic!("Unsupported reassignment to var, cant reassign var to this type"),
    }
}

fn change_val_by_expr(parser: &mut Parser, hash: u64, operator: Operator) {
    let expr = parser.get_expr();
    let var = parser.cache.get_var_from_hash(hash);
    if !var.mutable {
        panic!("Cant edit value of this variable");
    }

    let var_expr = var.to_expression();
    let bin_expr = Expr::BinExpr(BinExpr {
        left: Box::new(var_expr),
        op: operator,
        right: Box::new(expr), 
    });
    let value = eval_bin_expr(bin_expr);
    var.reassign_data_from_expr(value);
}

//when operator like += used
pub fn edit_var(parser: &mut Parser, hash: u64) {

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




