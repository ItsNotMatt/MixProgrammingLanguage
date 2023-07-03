use crate::{data_types, runtime::cache::Cache};

mod io;

pub fn import_io(cache: &mut Cache) {
    let func = data_types::NativeFunction::new("print".to_string(), Box::new(io::print));
    cache.add_native_fn(func);
}
