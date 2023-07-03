use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use std::hash::Hash;

use crate::ast::Expr;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Type {
    Int(IntType),
    String(StringType),
    Bool(BoolType),
    Struct,
    Enum,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct IntType {
    pub value: i32,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct StringType {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct BoolType {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Variable {
    pub name: String,
    pub hash: u64,
    pub data_type: Type, 
}

impl Variable {
    pub fn new(identifier: String, data_type: Type) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            data_type,
        }
    }

    pub fn to_expression(&mut self) -> Expr {
        match &self.data_type {
            Type::Int(i) => {
                return Expr::Number(i.value);
            }
            _ => {
                panic!("Cant convert type to expression");
            }
        }
    }

    pub fn reassign_data_from_expr(&mut self, expr: Expr) {
        println!("Reassigning data of {:?}, from expression {:?}", self.name, expr);
        match &mut self.data_type {
            Type::Int(i) => {
                match expr {
                    Expr::Number(n) => i.value = n,
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
    pub func: Box<dyn Fn()>,
    pub native: bool,
}

impl Function {
    pub fn new(identifier: String, func: Box<dyn Fn()>, native: bool) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            func,
            native,
        }
    }
}
