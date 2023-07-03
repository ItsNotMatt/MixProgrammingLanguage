#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{lexer::Token, ast::{Expr, BinExpr, Operator, Key, Identifier}, error::ParseError, evaluator::eval_bin_expr, runtime::cache::Cache};

mod variable;
mod function;
mod keyword;

pub struct Parser {
    tokens: Vec<Token>,
    expression: Option<Expr>,
    statements: Vec<Expr>,
    nest: usize,
    cache: Cache,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, cache: Cache) -> Self {
        Self {
            tokens,
            expression: None,
            statements: Vec::new(),
            nest: 0,
            cache,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.tokens.len() > 0 {
            return Some(self.tokens.remove(0));
        }
        return None
    }

    pub fn parse_tokens(&mut self, nest_start:  Option<usize>) {
        while let Some(token) = self.next_token() {
            println!("Parsing Token: {:?}", token);
            match token {
                Token::Identifier(s) => {
                    self.parse_identifier(s);
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
                Token::OCurly => {
                   self.nest += 1; 
                }
                Token::CCurly => {
                    self.nest -= 1;
                }
                Token::Eof => {
                    println!("End of file");
                    return;
                }
                _ => {
                    panic!("Cant parse token");
                }
            }
            if let Some(n) = nest_start {
                println!("Curr nest {}, start: {}", self.nest, n);
                if self.nest == n {
                    println!("breaking from current loop");
                    break;
                }
            } 
        }
    }

    fn parse_bin_expr(&mut self, expr: Option<Expr>) -> Expr {
        
        let mut bin_expr = Expr::BinExpr(BinExpr {
            left: Box::new(expr.unwrap_or_else(|| self.parse_expr().unwrap())),
            op: {
                match self.parse_expr().unwrap() {
                    Expr::Operator(op) => op,
                    _ => panic!("Expected operator"),
                }
            },
            right: Box::new(self.parse_expr().unwrap()),
        });

        bin_expr = eval_bin_expr(bin_expr);
        self.expression = Some(bin_expr.clone());

        if self.parse_next_expression() {
            println!("continuing to create next bin expr");
            self.parse_bin_expr(Some(bin_expr.clone()));
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

    fn get_expr(&mut self) -> Expr { //used when you want to check if expr ends or create a new exp
        if self.tokens[1] != Token::Semi {
            let expr = self.parse_bin_expr(None);
            return expr;
        }
        else {
            match self.next_token().unwrap() {
                Token::Number(n) => return Expr::Number(n),
                Token::Identifier(s) => {
                    let var = self.cache.get_var_from_string(&s);
                    return var.to_expression();
                }
                _ => panic!("Couldnt properly parse token for var"),
            }
        }
    }

    fn parse_next_expression(&mut self) -> bool {
        let token = &self.tokens[0];
        match token {
            Token::Operator(_) => return true,
            Token::Number(_) => return true,
            Token::OParen => return true,
            _ => return false,
        }
    }

    fn parse_keyword(&mut self, key: Key) {
        println!("Parsing key word: {:?}", key);
        
        match key {
            Key::Let => {
                variable::assign_var(self);
            }
            Key::If => {
                keyword::parse_if(self);
            }
            _ => {
                panic!("Unsupported key word");
            }
        }
    }

    fn parse_identifier(&mut self, identifier: String) {
        if let Some(hash) = self.cache.get_var_hash(&identifier) {
            variable::edit_var(self, hash);
        }
        else if let Some(hash) = self.cache.get_fn_hash(&identifier) {
            function::parse_function(self, hash);
        }
        else {
            panic!("Cant find identifier in this context.");
        }
    }


}

