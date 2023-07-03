use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::data_types::{Variable, Function, NativeFunction};

pub struct Cache {
    variables: HashMap<u64, Variable>,
    functions: HashMap<u64, Function>,
    native_functions: HashMap<u64, NativeFunction>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            native_functions: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, var: Variable) {
        println!("Cache recieved {:?}", var);
        self.variables.insert(var.hash, var);
    }

    pub fn get_var_from_string(&mut self, name: &String) -> &mut Variable {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if let Some(var) = self.variables.get_mut(&s.finish()) {
            return var;
        }
        else {
            panic!("Not able to get var from string");
        }
    }

    pub fn add_native_fn(&mut self, func: NativeFunction) {
        println!("Adding native func");
        self.native_functions.insert(func.hash, func);
    }

}

