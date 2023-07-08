use std::io;
use crate::ast::Expr;

pub fn print(args: Vec<Expr>) -> Option<Expr> {
    for arg in args {
        println!("{}", arg);
    }
    None
}

pub fn input(args: Vec<Expr>) -> Option<Expr> {
    let mut input = String::new();

    if args.len() > 1 {
        panic!("Function input only takes one argument");
    }
    else if args.len() > 0 {
        match &args[0] {
            Expr::String(s) => println!("{}", s),
            _ => panic!("Function takes argument of type String"),
        }
    }
    
    io::stdin().read_line(&mut input).expect("Failed to read input from user");
    println!("{}", input);

    Some(Expr::String(input.trim().to_string()))
}
