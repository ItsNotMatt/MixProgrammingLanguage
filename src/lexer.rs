use crate::ast::{Operator, Key, ArithmeticOperator, ComparisonOperator};

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: char,
    src: Vec<char>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    String(String),
    Identifier(String),
    Operator(Operator),
    Equal,
    Bang,
    Dot,
    Keyword(Key),
    OParen,
    CParen,
    OBracket,
    CBracket,
    OCurly,
    CCurly,
    Comma,
    Semi,
    Eof,
}

impl Lexer {
    pub fn new(src: Vec<char>) -> Self {
        Self {
            position: 0,
            read_position: 0,
            ch: '0',
            src,
        }
    }

    fn check_for_keyword(&mut self, identifier: &str) -> Option<Token> {
        match identifier {
            "let" => {
                return Some(Token::Keyword(Key::Let));
            }
            "if" => {
                return Some(Token::Keyword(Key::If));
            }
            "else" => {
                return Some(Token::Keyword(Key::Else));
            }
            "for" => {
                return Some(Token::Keyword(Key::For));
            }
            "while" => {
                return Some(Token::Keyword(Key::While));
            }
            "true" => {
                return Some(Token::Keyword(Key::True));
            }           
            "false" => {
                return Some(Token::Keyword(Key::False));
            }
            "break" => {
                return Some(Token::Keyword(Key::Break));
            }
            _ => {
                return None;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        self.read_position = self.position;
        while self.position < self.src.len() {
            self.ch = self.src[self.position];

            if !self.ch.is_alphabetic() && self.ch != '_' {
                if self.ch.is_numeric(){
                    panic!("Cant put numbers in identifier");
                }

                let slice = self.src[self.read_position - 1..self.position].to_vec();
                let identifier: String = slice.iter().collect();

                if let Some(keyword) = self.check_for_keyword(&identifier) {
                    return keyword;
                }
                else {
                    return Token::Identifier(identifier);
                }
            }
            self.position += 1;
        }
        panic!();
    }

    fn read_num(&mut self) -> Token {
        self.read_position = self.position;
        while self.position < self.src.len() {
            self.ch = self.src[self.position];

            if !self.ch.is_numeric() {
                if self.ch.is_alphabetic(){
                    panic!("Cant put letters in number");
                }

                let slice = self.src[self.read_position - 1..self.position].to_vec();
                let nums: Vec<i32> = slice.iter().filter_map(|&c| c.to_digit(10)).map(|d| d as i32).collect();
                let num: i32 = nums.iter().fold(0, |acc, &d| acc * 10 + d);
                return Token::Number(num);
            }
            self.position += 1;
        }
        panic!();
    }

    fn read_string(&mut self) -> Token {
        self.read_position = self.position;
        while self.position < self.src.len() {
            self.ch = self.src[self.position];
            if self.ch == '"' {
                println!("ch is quote");
                let slice = self.src[self.read_position..self.position].to_vec();
                let string: String = slice.iter().collect();
                self.position += 1;
                return Token::String(string);
            }
            self.position += 1;
        }
        panic!();
    }

    fn check_ahead(&mut self, check: char) -> bool {
        if check == self.src[self.position] {
            self.position += 1;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens:Vec<Token> = Vec::new();

        while self.position < self.src.len() {
            self.ch = self.src[self.position];
            self.position += 1;

            match self.ch {
                '=' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::DoubleEqual)));
                    } 
                    else {
                        tokens.push(Token::Equal);
                    }
                }
                ';' => {
                    tokens.push(Token::Semi);
                }
                ',' => {
                    tokens.push(Token::Comma);
                }
                '.' => {
                    tokens.push(Token::Dot);
                }
                '(' => {
                    tokens.push(Token::OParen);
                }
                ')' => {
                    tokens.push(Token::CParen);
                }
                '{' => {
                    tokens.push(Token::OCurly);
                }
                '}' => {
                    tokens.push(Token::CCurly);
                }
                '"' => { 
                    tokens.push(self.read_string());
                }
                '!' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::NotEqual)));
                    } 
                    else {
                        tokens.push(Token::Bang);
                    }
                }
                '+' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::AddEq)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Add)));
                    }
                }
                '-' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::SubEq)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Sub)));
                    }
                }
                '*' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::MultiEq)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Multi)));
                    }
                }
                '/' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::DivEq)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Div)));
                    }
                }
                '>' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::GreaterEqual)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::Greater)));
                    }
                }
                '<' => {
                    if self.check_ahead('=') {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::LessEqual)));
                    } 
                    else {
                        tokens.push(Token::Operator(Operator::Comparison(ComparisonOperator::Less)));
                    }
                }
                _ => {
                    if self.ch.is_whitespace(){
                        continue;
                    }
                    else if self.ch.is_alphabetic() || self.ch == '_' {
                        tokens.push(self.read_identifier());
                    }
                    else if self.ch.is_numeric() {
                        tokens.push(self.read_num());
                    }
                    else {
                        panic!("Cant identify token!");
                    }
                }
            }
        }
        tokens.push(Token::Eof);
        return tokens;
    }
}


