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
}

pub struct Function { 
    name: String,
    pub hash: u64,
    pub func: Box<dyn Fn()>,
}

impl Function {
    pub fn new(identifier: String, func: Box<dyn Fn()>) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            func,
        }
    }
}

pub struct NativeFunction { 
    name: String,
    pub hash: u64,
    pub func: Box<dyn Fn()>,
}

impl NativeFunction {
    pub fn new(identifier: String, func: Box<dyn Fn()>) -> Self {
        let mut s = DefaultHasher::new();
        identifier.hash(&mut s);
        Self {
            name: identifier,
            hash: s.finish(),
            func,
        }
    }
}
