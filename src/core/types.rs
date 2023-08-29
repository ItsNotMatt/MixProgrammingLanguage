use crate::ast::Expr;

//Edit Variable
pub fn move_var(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match &args[0] {
        Expr::Identifier(s) => {
        }
        _ => panic!("Cant use method move on this type"),
    }
    None
}


//Conversions
pub fn to_int(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match &args[0] {
        Expr::String(s) => {
            let num: i32 = s.parse().expect("Cant convert this type to int");
            return Some(Expr::Number(num));
        }
        _ => { panic!("Cant use method to_int on this type"); }
    }
}

pub fn to_string(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 1 { panic!("This function only takes one argument of self"); }
    match args[0] {
        Expr::Number(n) => {
            let str = n.to_string();
            return Some(Expr::String(str));
        }
        _ => { panic!("Cant use method to_string on this type"); }
    }
}
