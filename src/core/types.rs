use crate::ast::Expr;

pub fn to_int(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match &args[0] {
        Expr::String(s) => {
            println!("Converting to int");
            let num: i32 = s.parse().unwrap();
            return Some(Expr::Number(num));
        }
        _ => { panic!("Cant use method to_int on this type"); }
    }
}

pub fn to_string(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match args[0] {
        Expr::Number(n) => {
            println!("Converting to string");
            let str = n.to_string();
            return Some(Expr::String(str));
        }
        _ => { panic!("Cant use method to_string on this type"); }
    }
}
