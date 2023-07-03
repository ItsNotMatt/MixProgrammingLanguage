use crate::ast::{Expr, Operator, ArithmeticOperator, ComparisonOperator};

pub fn eval_bin_expr(expr: Expr) -> Expr  {
    if let Expr::BinExpr(exp) = expr {
        match exp.op {
            Operator::Arithmetic(op) => {
                let ex = arithmetic(&exp.left, &exp.right, op);
                return ex;
            }
            Operator::Comparison(op) => {
                let ex = compare(&exp.left, &exp.right, op);
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
                _ => {
                    panic!("Operation not supported");
                }
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
                ComparisonOperator::Greater => {
                    if l > r {
                        return Expr::Bool(true);
                    }
                }
                ComparisonOperator::Less => {
                    if l > r {
                        return Expr::Bool(true);
                    }
                }
                ComparisonOperator::DoubleEqual => {
                    if l == r {
                        return Expr::Bool(true);
                    }
                }
                ComparisonOperator::LessEqual => {
                    if l <= r {
                        return Expr::Bool(true);
                    }
                }
                ComparisonOperator::GreaterEqual => {
                    if l >= r {
                        return Expr::Bool(true);
                    }
                }
            }
        }
        _ => panic!("Cant compare expression types"),
    }
    return Expr::Bool(false);
}


