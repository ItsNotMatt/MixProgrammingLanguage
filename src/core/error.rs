use crate::ast::Expr;

pub fn panic(args: Vec<Expr>) -> Option<Expr> {
    for arg in args {
        println!("{}", arg);
    }
    panic!();
}
