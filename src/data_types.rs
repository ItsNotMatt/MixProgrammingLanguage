use std::collections::HashMap;
use std::ops::Range;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use std::hash::Hash;

use crate::ast::{Expr, Key};
use crate::lib;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Type {
    Int(i32),
    String(String),
    Bool(bool),
    Array(Box<Vec<Expr>>),
    Struct,
    Enum(Enum),
}


#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Variable {
    pub name: String,
    pub hash: u64,
    pub data_type: Type, 
    pub mutable: bool,
}

impl Variable {
    pub fn new(identifier: String, data_type: Type, mutable: bool) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            data_type,
            mutable,
        }
    }

    pub fn to_expression(&mut self) -> Expr {
        match &self.data_type {
            Type::Int(i) => {
                return Expr::Number(*i);
            }
            Type::String(s) => {
                return Expr::String(s.clone());
            }
            Type::Bool(b) => {
                return Expr::Bool(*b);
            }
            Type::Array(a) => {
                return Expr::Array(a.clone());
            }
//            Type::Enum(a) => {
//                return Expr::Enum();
//            }
            _ => {
                panic!("Cant convert type to expression");
            }
        }
    }

    pub fn index_to_expression(&mut self, index: usize) -> Expr {
        match &self.data_type {
            Type::Array(a) => {
                return a[index].clone();
            }
            _ => panic!("Cant convert this type to expression"),
        }
    }

    pub fn reassign_data_from_expr(&mut self, expr: Expr) {
        println!("\n----Reassigning data of {:?}, from expression {:?}----\n", self.name, expr);
        match &mut self.data_type {
            Type::Int(i) => {
                match expr {
                    Expr::Number(n) => *i = n,
                    _ => panic!("Reassigning var from expr not supported"),
                }
            }
            _ => panic!("Reassigning this var from expr of that type not supported"),
        }
    }
}

pub struct Function { 
    pub name: String,
    pub hash: u64,
    pub func: Box<dyn Fn(Vec<Expr>) -> Option<Expr>>,
}

impl Function {
    pub fn new(identifier: String, func: Box<dyn Fn(Vec<Expr>) -> Option<Expr>>) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            func,
        }
    }
}

pub struct CustomFunction {
    pub name: String,
    pub hash: u64,
    pub variables: HashMap<u64, TempVar>,
    pub body: Range<usize>,
    pub return_val: Option<Key>
}

impl CustomFunction {
    pub fn new(identifier: String, variables: HashMap<u64, TempVar>, body: Range<usize>, return_val: Option<Key>) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            variables,
            body,
            return_val,
        }
    }
}

#[derive(Debug)]
pub struct TempVar {
    pub name: String,
    pub data_type: Option<Type>,
    pub type_requirement: Key,
}

impl TempVar {
    pub fn new(name: String, type_requirement: Key) -> Self {
        Self {
            name,
            data_type: None,
            type_requirement,
        }
    }

    pub fn convert_to_var(&mut self) -> Variable {
        Variable::new(self.name.clone(), self.data_type.clone().unwrap(), true)
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Enum {
    pub name: String,
    pub hash: u64,
    pub options: Vec<String>,
}

impl Enum {
    pub fn new(name: String, options: Vec<String>) -> Enum {
        let hash = lib::get_hash(&name);
        Self {
            name,
            hash,
            options,
        }
    }
}

