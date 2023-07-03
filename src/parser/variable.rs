use crate::{lexer::Token, ast::Expr, data_types::{Type, IntType, Variable}, parser::variable, runtime};

use super::Parser;

fn create_var_from_expr(parser: &mut Parser, identifier: String, expr: Expr) {
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
                    let token = parser.next_token().unwrap();
                    let mut expr = match token {
                        Token::Number(n) => Expr::Number(n),
                        Token::Identifier(s) => {
                            let var = parser.cache.get_var_from_string(&s);
                            let expr = var.to_expression();
                            expr
                        },
                        _ => panic!("Token after '=' isnt valid expression"),
                    };
                    if parser.tokens[0] != Token::Semi {
                        println!("made new bin expr: {:?}", expr);
                        expr = parser.parse_bin_expr(Some(expr));
                    }
                    create_var_from_expr(parser, s, expr);
                }

                _ => {
                    panic!("Illegal token after identifier {}", s);
                }
            }
        }
        _ => {
            panic!("Illegal token after Let");
        }
    }
}
