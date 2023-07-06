use std::any::Any;

use crate::ast::Expr;

pub fn print(args: Box<dyn Any>) {
    println!("\n--Args passed into function--");
    if let Some(args) = args.downcast_ref::<Vec<Expr>>() {
        for arg in args {
            println!("{}", arg);
        }
    }
    else {
        panic!("Invalid argument types passed into function");
    }
    
}
