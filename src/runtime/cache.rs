use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::data_types::{Variable, Function, CustomFunction};

pub struct Cache {
    variables: HashMap<u64, Variable>,
    functions: HashMap<u64, Function>,
    custom_functions: HashMap<u64, CustomFunction>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            custom_functions: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, var: Variable) {
        println!("Cache recieved {:?}", var);
        self.variables.insert(var.hash, var);
    }

    pub fn get_var_from_string(&mut self, name: &String) -> Option<&mut Variable> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if let Some(var) = self.variables.get_mut(&s.finish()) {
            return Some(var);
        }
        else {
            return None;
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

    //used when you want to grab a hash to a var to pass it to multiple places
    pub fn get_var_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.variables.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            return None;
        }
    }


    //functions
    pub fn add_fn(&mut self, func: Function) {
        self.functions.insert(func.hash, func);
    }

    pub fn add_custom_fn(&mut self, func: CustomFunction) {
        println!("Adding func: {}, custom. Vars: {:?}", func.name, func.variables);
        for t in &func.body {
            println!("Func body: {:?}", t);
        }
        self.custom_functions.insert(func.hash, func);
    }

    pub fn get_fn_from_hash(&mut self, hash: u64) -> &mut Function {
        if let Some(func) = self.functions.get_mut(&hash) {
            return func;
        }
        else {
            panic!("Not able to get func from hash");
        }
    }

    //used when you want to grab a hash to a fn to pass it to multiple places
    pub fn get_fn_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.functions.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            return None;
        }
    }
}

