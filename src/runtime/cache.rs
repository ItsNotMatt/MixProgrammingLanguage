use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::{data_types::{Variable, Function, CustomFunction, Enum, self}, lib};

pub struct Cache {
    variables: HashMap<u64, Variable>,
    functions: HashMap<u64, Function>,
    custom_functions: HashMap<u64, CustomFunction>,
    enums: HashMap<u64, data_types::Enum>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            custom_functions: HashMap::new(),
            enums: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, var: Variable) {
        self.variables.insert(var.hash, var);
    }

    pub fn remove_temps(&mut self, hashs: Vec<u64>) {
        let mut temp_vars: Vec<u64> = Vec::new();
        for hash in hashs {
            if self.variables.contains_key(&hash) {
                temp_vars.push(hash);
            }
        }
        for hash in temp_vars {
            self.variables.remove(&hash);
        }
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

    pub fn add_custom(&mut self, func: CustomFunction) {
        println!("Adding func: {}, custom. Vars: {:?}", func.name, func.variables);
        self.custom_functions.insert(func.hash, func);
    }

    pub fn get_fn_from_hash(&mut self, hash: u64) -> &mut Function {
        if let Some(func) = self.functions.get_mut(&hash) {
            return func;
        }
        else { panic!("Not able to get func from hash"); }
    }

    pub fn get_custom_from_hash(&mut self, hash: u64) -> &mut CustomFunction {
        if let Some(func) = self.custom_functions.get_mut(&hash) {
            return func;
        }
        else { panic!("Not able to get func from hash"); }
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

    pub fn get_custom_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.custom_functions.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            return None;
        }
    }


    pub fn add_enum(&mut self, enm: data_types::Enum) {
        println!("Cache received enum {}", &enm.name);
        self.enums.insert(enm.hash, enm);
    }

    pub fn get_enum_from_string(&mut self, name: &String) -> Option<&data_types::Enum> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        let enm = self.enums.get(&s.finish());
        return enm;
    }

    pub fn get_enum_hash(&mut self, name: &String) -> Option<u64> {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        if self.enums.get(&s.finish()).is_some() {
            return Some(s.finish());
        }
        else {
            return None;
        }
    }
}

