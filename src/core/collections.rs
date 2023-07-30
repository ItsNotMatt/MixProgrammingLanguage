use crate::{ast::Expr, lib, runtime::cache::Cache, data_types};

pub fn import_collections(cache: &mut Cache) {
    let func = data_types::Function::new("len".to_string(),
        Box::new(len as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);
}


pub fn len(mut args: Vec<Expr>) -> Option<Expr> {
    println!("Getting length");

    lib::validate_len(args.len(), 1);

    match args.remove(0) {
        Expr::Array(a) => {
            return Some(Expr::Number(a.len() as i32));
        }
        _ => panic!("Cant use len function on this type"),
    }
}
