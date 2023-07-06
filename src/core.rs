use std::any::Any;

use crate::{data_types, runtime::cache::Cache};

mod io;

pub fn import_default_functions(cache: &mut Cache) {
    let func = data_types::Function::new("print".to_string(),
        Some(Box::new(io::print as fn(Box<dyn Any>))));
    cache.add_fn(func);
    let func = data_types::Function::new("test".to_string(),
        Some(Box::new(test as fn(Box<dyn Any>))));
    cache.add_fn(func);
}

pub fn ensure_no_args(args: Box<dyn Any>) {
    if let Some(arg) = args.downcast_ref::<i32>() {
        if *arg != -69420 {
            panic!("No args expected smh");
        }
        return;
    }
    panic!("No args expected");
}

fn test(args: Box<dyn Any>) {
    ensure_no_args(args);
    println!("test");
}

pub fn import_io(_cache: &mut Cache) {

}
