use crate::data_types::StringType;
use crate::ast::Expr;

pub fn to_int(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match args[0] {
        Expr::String(_) => {
            println!("Converting to int");
            return Some(Expr::Number(10));
        }
        _ => { panic!("Cant use method to_int on this type"); }
    }
}

pub fn to_string(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match args[0] {
        Expr::Number(_) => {
            println!("Converting to string");
            return Some(Expr::String("test".to_string()));
        }
        _ => { panic!("Cant use method to_string on this type"); }
    }
}
