use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, result};

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

    pub fn get_var_from_hash(&mut self, hash: u64) -> &mut Variable {
        if let Some(var) = self.variables.get_mut(&hash) {
            return var;
        }
        else {
            panic!("Not able to get var from hash");
        }
    }

    pub fn get_var_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.variables.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            println!("Not able to get hash from string");
            return None;
        }
    }




    pub fn get_fn_from_hash(&mut self, hash: u64) -> &mut Function {
        if let Some(func) = self.functions.get_mut(&hash) {
            return func;
        }
        else {
            panic!("Not able to get func from hash");
        }
    }

    pub fn get_fn_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.functions.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            println!("Not able to get hash from string");
            return None;
        }
    }




    pub fn add_native_fn(&mut self, func: NativeFunction) {
        println!("Adding native func");
        self.native_functions.insert(func.hash, func);
    }

    pub fn get_native_fn_from_hash(&mut self, hash: u64) -> &mut NativeFunction {
        if let Some(func) = self.native_functions.get_mut(&hash) {
            return func;
        }
        else {
            panic!("Not able to get native func from hash");
        }
    }

    pub fn get_native_fn_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.native_functions.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            println!("Not able to get hash from string");
            return None;
        }
    }


}

