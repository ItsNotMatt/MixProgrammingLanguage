use crate::{ast::Expr, parser::{self, Parser}};

pub fn print(args: Vec<Expr>) -> Option<Expr> {
    for arg in args {
        println!("{}", arg);
    }
    None
}

pub fn input(args: Vec<Expr>) -> Option<Expr> {
    let input = String::new();
    println!("{}", input);
    Some(Expr::Number(0))
}
