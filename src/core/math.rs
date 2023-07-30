use crate::{ast::Expr, runtime::cache::Cache, data_types, lib};


pub fn import_math(cache: &mut Cache) {
    let func = data_types::Function::new("add_two_nums".to_string(),
        Box::new(add_two_nums as fn(Vec<Expr>) -> Option<Expr>));
    cache.add_fn(func);
}

pub fn add_two_nums(mut args: Vec<Expr>) -> Option<Expr> {
    lib::validate_len(args.len(), 2);

    let mut res = 0;
    match args.remove(0) {
        Expr::Number(n) => res += n,
        _ => panic!("Expected int type"),
    }
    match args.remove(0) {
        Expr::Number(n) => res += n,
        _ => panic!("Expected int type"),
    }
    Some(Expr::Number(res))
}


pub fn random_range(mut args: Vec<Expr>) -> Option<Expr> {
    if args.len() != 2 {
        panic!("Function takes 2 arguments, {} were passed", args.len());
    }
    let mut min = 0;
    let mut max = 0;

    let arg = args.remove(0);
    match arg {
        Expr::Number(n) => min = n,
        _ => panic!("Invalid arg. Function takes a number"),
    }
    let arg = args.remove(0);
    match arg {
        Expr::Number(n) => max = n,
        _ => panic!("Invalid arg. Function takes a number"),
    }

    None
}

