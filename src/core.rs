use crate::{data_types, runtime::cache::Cache, ast::Expr};

mod io;
mod types;
mod error;
mod math;
mod collections;

pub fn import_module(module: &str, cache: &mut Cache) {
    match module {
        "core" => import_default_functions(cache),
        "math" => math::import_math(cache),
        "collections" => collections::import_collections(cache),
        _ => panic!("Cant find module"),
    }
}

fn import_default_functions(cache: &mut Cache) {
    let func = data_types::Function::new("print".to_string(),
        Box::new(io::print as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);
    let func = data_types::Function::new("input".to_string(),
        Box::new(io::input as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);

    let func = data_types::Function::new("to_int".to_string(),
        Box::new(types::to_int as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);   
    let func = data_types::Function::new("to_string".to_string(),
        Box::new(types::to_string as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);

    let func = data_types::Function::new("panic".to_string(),
        Box::new(error::panic as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);


    let func = data_types::Function::new("test".to_string(),
        Box::new(test as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);
}

pub fn ensure_no_args() {
}

fn test(args: Vec<Expr>) -> Option<Expr> {
    if args.len() > 0 {
        panic!("This function doesnt take any arguments.");
    }
    None
}

pub fn import_io(_cache: &mut Cache) {

}
