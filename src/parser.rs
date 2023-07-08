#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{lexer::Token, ast::{Expr, BinExpr, Operator, Key, Identifier}, error::ParseError, evaluator::eval_bin_expr, runtime::cache::Cache};

mod variable;
mod function;
mod keyword;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    read_position: usize,
    consume_tokens: bool,
    expression: Option<Expr>,
    statements: Vec<Expr>,
    nest: usize,
    pub cache: Cache,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, cache: Cache) -> Self {
        Self {
            tokens,
            position: 0,
            read_position: 0,
            consume_tokens: true,
            expression: None,
            statements: Vec::new(),
            nest: 0,
            cache,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.tokens.len() > 0 {
            if self.consume_tokens {
                println!("--Consuming Token: {:?}", self.tokens[0].clone());
                return Some(self.tokens.remove(0));
            }
            else {
                let token = self.tokens[self.position].clone();
                self.position += 1;
                println!("--Copying token: {:?}, position: {}", token, self.position);
                return Some(token);
            }
        }
        return None
    }

    fn peek_token(&mut self) -> Option<&Token> {
        if self.tokens.len() > 0 {
            if self.consume_tokens {
                println!("--Peeking at Token: {:?}", &self.tokens[0]);
                return Some(&self.tokens[0]);
            }
            else {
                println!("--Peeking at Token: {:?}, position: {}", &self.tokens[self.position], self.position);
                return Some(&self.tokens[self.position]);
            }
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
                println!("   Curr nest {}, start: {}", self.nest, n);
                if self.nest == n {
                    println!("breaking from current loop");
                    break;
                }
            } 
        }
    }

    fn parse_bin_expr(&mut self, expr: Option<Expr>) -> Expr {
        println!("Making bin expr");
        
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
        println!("Matching on {:?}, Position: {}", token, self.position);

        match token {
            Token::Number(n) => return Ok(Expr::Number(n)),
            Token::Keyword(k) => {
                match k {
                    Key::True => return Ok(Expr::Bool(true)),
                    Key::False => return Ok(Expr::Bool(false)),
                    _ => panic!("Cant return expression from this keyword"),
                }
            }
            Token::String(s) => return Ok(Expr::String(s)),
            Token::Identifier(s) => {
                let var = self.cache.get_var_from_string(&s);
                let expr = var.to_expression();
                return Ok(expr);
            }
            Token::Operator(op) => return Ok(Expr::Operator(op)),
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
        let exp = self.parse_expr().unwrap();
        
        if self.parse_next_expression() {
            let expr = self.parse_bin_expr(Some(exp));
            return expr;
        }
        else {
            return exp;
       }
    }

    fn parse_next_expression(&mut self) -> bool {
        let token = self.peek_token().unwrap();
        println!("Check token ahead: {:?}", token);
        match token {
            Token::Operator(_) => return true,
            Token::Number(_) => return true,
            Token::String(_) => return true,
            Token::Identifier(_) => return true,
            Token::OParen => return true,
            _ => return false,
        }
    }

    fn parse_keyword(&mut self, key: Key) {
        println!("\n---Parsing key word: {:?}", key);
        
        match key {
            Key::Let => {
                variable::assign_var(self);
            }
            Key::If => {
                keyword::parse_if(self);
            }
            Key::While => {
                keyword::parse_while(self, None);
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

