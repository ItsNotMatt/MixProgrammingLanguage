#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{lexer::Token, ast::{Expr, BinExpr, Operator, Key, Identifier}, error::ParseError, evaluator::eval_bin_expr, runtime::cache::Cache};

mod variable;

pub struct Parser {
    tokens: Vec<Token>,
    expression: Option<Expr>,
    cache: Cache,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, cache: Cache) -> Self {
        Self {
            tokens,
            expression: None,
            cache,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.tokens.len() > 0 {
            return Some(self.tokens.remove(0));
        }
        return None
    }

    pub fn parse_tokens(&mut self) {
        while let Some(token) = self.next_token() {
            println!("Parsing Token: {:?}", token);
            match token {
                Token::Identifier(s) => {
                    match self.parse_identifier(s) {
                        Identifier::Variable(hash) => {
                            let var = self.cache.get_var_from_hash(hash);
                            println!("Found variable {:?}", var);
                        }
                        Identifier::Fn(hash) => {
                            let func = self.cache.get_fn_from_hash(hash);
                            println!("Found fn");
                        }
                        Identifier::NativeFn(hash) => {
                            let func = self.cache.get_native_fn_from_hash(hash);
                            println!("Found native fn");
                        }
                    }
                }
                Token::Keyword(k) => {
                    self.parse_keyword(k);
                }
                Token::Number(n) => {
                    self.parse_bin_expr(Some(Expr::Number(n)));
                }
                Token::Semi => {
                    self.expression = None;
                    //creates statement and ends last expression
                }
                Token::CParen => {
                    continue;
                }
                Token::Eof => {
                    println!("End of file");
                    return;
                }
                _ => {
                    panic!("Cant parse token");
                }
            }
        }
    }

    fn parse_bin_expr(&mut self, expr: Option<Expr>) -> Expr {
        
        let mut bin_expr = Expr::BinExpr(BinExpr {
            left: dbg!(Box::new(expr.unwrap_or_else(|| self.parse_expr().unwrap()))),
            op: {
                match self.parse_expr().unwrap() {
                    Expr::Operator(op) => op,
                    _ => panic!("Expected operator"),
                }
            },
            right: dbg!(Box::new(self.parse_expr().unwrap())),
        });

        bin_expr = eval_bin_expr(bin_expr);
        self.expression = Some(bin_expr.clone());
        let token = &self.tokens[0]; 
        println!("next tok: {:?}", token);
        if token != &Token::CParen && token != &Token::Semi {
            println!("continuing to create next bin expr");
            self.parse_bin_expr(Some(bin_expr.clone()));
        } 
        else {
            println!("Finished binexpr: {}", bin_expr);
        }
        return bin_expr; //for cases when I want a return value 
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.next_token().unwrap();
        println!("Matching on {:?}", token);

        match token {
            Token::Number(n) => {
                return Ok(Expr::Number(n))
            }
            Token::Identifier(s) => {
                let var = self.cache.get_var_from_string(&s);
                let expr = var.to_expression();
                return Ok(expr);
            }
            Token::Operator(op) => {
                return Ok(Expr::Operator(op))
            }
            Token::OParen => {
                self.parse_bin_expr(None);
                return Ok(self.expression.clone().unwrap());
            }
            Token::CParen => {
                return Err(ParseError::Error("Close paren at wrong moment".to_string()));
            }
            _ => {
                return Err(ParseError::Error("cant parse token to expr".to_string()));
            }
        }
    }

    fn parse_keyword(&mut self, key: Key) {
        println!("Parsing key word: {:?}", key);
        
        match key {
            Key::Let => {
                variable::assign_var(self);
            }
            _ => {
                panic!("Unsupported key word");
            }
        }
    }

    fn parse_identifier(&mut self, identifier: String) -> Identifier {
        if let Some(hash) = self.cache.get_var_hash(&identifier) {
            return Identifier::Variable(hash);
        }
        else if let Some(hash) = self.cache.get_fn_hash(&identifier) {
            return Identifier::Fn(hash);
        }
        else if let Some(hash) = self.cache.get_native_fn_hash(&identifier) {
            return Identifier::NativeFn(hash);
        }
        else {
            panic!("Cant find identifier in this context.");
        }
    }


}

