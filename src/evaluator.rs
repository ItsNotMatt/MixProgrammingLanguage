use crate::ast::{Expr, Operator, ArithmeticOperator, ComparisonOperator};

pub fn eval_bin_expr(expr: Expr) -> Expr  {
    if let Expr::BinExpr(exp) = expr {
        match exp.op {
            Operator::Arithmetic(op) => {
                let ex = arithmetic(&exp.left, &exp.right, op);
                println!("Exp after eval: {}", ex);
                return ex;
            }
            Operator::Comparison(op) => {
                let ex = compare(&exp.left, &exp.right, op);
                println!("Exp after eval: {}", ex);
                return ex;
            }
        }
    }
    else {
        panic!("Cant eval non Binary expression");
    }
}

fn arithmetic(left: &Expr, right: &Expr, op: ArithmeticOperator) -> Expr {
    match (left, right) {
        (Expr::Number(l), Expr::Number(r)) => {
            match op {
                ArithmeticOperator::Add => {
                    let res = l + r;
                    return Expr::Number(res);
                }
                ArithmeticOperator::Sub => {
                    let res = l - r;
                    return Expr::Number(res);
                }
                _ => panic!("Operation not supported"),
            }
        }
        (Expr::String(l), Expr::String(r)) => {
            match op {
                ArithmeticOperator::Add => {
                    let combined = format!("{}{}", l, r);
                    return Expr::String(combined);
                }
                _ => panic!("Operation not supported"),
            }
        }
        _ => {
            panic!("Operand on these types are not supported");
        }
    }
}

pub fn compare(left: &Expr, right: &Expr, op: ComparisonOperator) -> Expr {
    match (left, right) {
        (Expr::Number(l), Expr::Number(r)) => {
            match op {
                ComparisonOperator::Greater => return Expr::Bool(l > r),
                ComparisonOperator::Less => return Expr::Bool(l < r),
                ComparisonOperator::LessEqual => return Expr::Bool(l <= r),
                ComparisonOperator::GreaterEqual => return Expr::Bool(l >= r),
                ComparisonOperator::DoubleEqual => return Expr::Bool(l == r), 
                ComparisonOperator::NotEqual => return Expr::Bool(l != r), 
            }
        }
        (Expr::Bool(l), Expr::Bool(r)) => {
            match op {
                ComparisonOperator::DoubleEqual => return Expr::Bool(l == r),
                ComparisonOperator::NotEqual => return Expr::Bool(l != r),
                _ => panic!("Cant compare bools with this operand"),
            }
        }
        (Expr::String(l), Expr::String(r)) => {
            match op {
                ComparisonOperator::DoubleEqual => return Expr::Bool(l == r),
                ComparisonOperator::NotEqual => return Expr::Bool(l != r),
                _ => panic!("Cant compare bools with this operand"),
            }
        }
        _ => panic!("Cant compare expression types"),
    }
}


