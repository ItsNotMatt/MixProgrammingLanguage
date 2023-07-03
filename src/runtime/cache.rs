use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::data_types::{Variable, Function, BuiltInFunction};

pub struct Cache {
    variables: HashMap<u64, Variable>,
    functions: HashMap<u64, Function>,
    built_in_functions: HashMap<u64, BuiltInFunction>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            built_in_functions: HashMap::new(),
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

}

