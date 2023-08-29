use std::fmt;

use crate::data_types;




pub enum Identifier {
    Variable(u64),
    Fn(u64),
    NativeFn(u64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Number(i32),
    Bool(bool),
    String(String),
    //when its still in var form
    Identifier(String), 
    Operator(Operator),
    BinExpr(BinExpr),
    Array(Box<Vec<Expr>>),
    Enum(data_types::Enum),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{{{}}}", b),
            Expr::String(s) => write!(f, "{}", s),
            Expr::Identifier(s) => write!(f, "{{{:?}}}", s),
            Expr::Operator(o) => write!(f, "{}", o),
            Expr::BinExpr(e) => write!(f, " [{} {} {}]", e.left, e.op, e.right), 
            Expr::Array(a) => write!(f, "{{{:?}}}", a),
            Expr::Enum(e) => write!(f, "{{{:?}}}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BinExpr {
    pub left : Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
}


//////////////////////////////////////////////////
//Tokens
//////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Key {
    For,
    Fn,
    Let,
    While,
    If,
    Else,
    Int,
    String,
    Bool,
    Const,
    True,
    False,
    Return,
    Break,
    Thread,
    Import,
    Enum,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Comparison(ComparisonOperator),
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Arithmetic(o) => write!(f, "{}", o),
            Operator::Comparison(c) => write!(f, "{:?}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Multi,
    Div,
    AddEq,
    SubEq,
    MultiEq,
    DivEq,
    Pow
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticOperator::Add => write!(f, "+"),
            ArithmeticOperator::Sub => write!(f, "-"),
            _ => write!(f, "not implemented yet"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ComparisonOperator {
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    DoubleEqual,
    NotEqual,
}
