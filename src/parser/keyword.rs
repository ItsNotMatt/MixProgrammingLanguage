use crate::ast::Expr;

use super::Parser;

pub fn parse_if(parser: &mut Parser) {
    let expr = parser.get_expr() ;
    match expr {
        Expr::Bool(b) => {
            if b {
                println!("statement is true, entering if statement");
            }
            else {
                println!("statement is false");
            }
        }
        _ => panic!("Expression isnt a boolean"),
    }
}
