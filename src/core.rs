use std::any::Any;

use crate::{data_types, runtime::cache::Cache, ast::Expr};

mod io;

pub fn import_io(cache: &mut Cache) {
    let func = data_types::Function::new("print".to_string(), Some(Box::new(io::print as fn(Box<dyn Any>))));
    cache.add_fn(func);
}
