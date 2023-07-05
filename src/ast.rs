use std::fmt;




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
    Identifier(String), 
    Operator(Operator),
    BinExpr(BinExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{{{}}}", n),
            Expr::String(s) => write!(f, "{{{}}}", s),
            Expr::Identifier(s) => write!(f, "{{{:?}}}", s),
            Expr::Operator(o) => write!(f, "{{{}}}", o),
            Expr::BinExpr(e) => write!(f, " [{} {} {}]", e.left, e.op, e.right), 
            Expr::Bool(b) => write!(f, "{{{}}}", b),
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
    Let,
    While,
    If,
    Else,
    True,
    False,
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
    DoubleEqual,
    Greater,
    LessEqual,
    GreaterEqual
}
