
#[derive(Debug)]
pub enum KeyWord {
    For,
    Let,
    While,
    If,
    Else,
    True,
    False,
}


#[derive(Debug)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Equals,
    Comparison(ComparisonOperator),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ComparisonOperator {
    Less,
    Greater,
    LessEqual,
    GreaterEqual
}
