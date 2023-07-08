use std::any::Any;

use crate::{data_types, runtime::cache::Cache, ast::Expr, parser::Parser};

mod io;

pub fn import_default_functions(cache: &mut Cache) {
    let func = data_types::Function::new("print".to_string(),
        Some(Box::new(io::print as fn(Vec<Expr>) -> Option<Expr>)));
    cache.add_fn(func);
    let func = data_types::Function::new("input".to_string(),
        Some(Box::new(io::input as fn(Vec<Expr>) -> Option<Expr>)));
    cache.add_fn(func);

    let func = data_types::Function::new("test".to_string(),
        Some(Box::new(test as fn(Vec<Expr>) -> Option<Expr>)));
    cache.add_fn(func);
}

pub fn ensure_no_args() {
}

fn test(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 0 {
        panic!("This function doesnt take any arguments.");
    }
    println!("test");
    None
}

pub fn import_io(_cache: &mut Cache) {

}
