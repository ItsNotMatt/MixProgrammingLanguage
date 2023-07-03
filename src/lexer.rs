use crate::ast::{Operator, Key, ArithmeticOperator};

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: char,
    src: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Operator(Operator),
    Equal,
    Keyword(Key),
    OParen,
    CParen,
    OBracket,
    CBracket,
    OCurly,
    CCurly,
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
            _ => {
                return None;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        self.read_position = self.position;
        while self.position < self.src.len() {
            self.ch = self.src[self.position];

            if !self.ch.is_alphabetic() {
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens:Vec<Token> = Vec::new();

        while self.position < self.src.len() {
            self.ch = self.src[self.position];
            self.position += 1;

            match self.ch {
                '(' => {
                    tokens.push(Token::OParen);
                }
                ')' => {
                    tokens.push(Token::CParen);
                }
                '+' => {
                    tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Add)));
                }
                '-' => {
                    tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Sub)));
                }
                '*' => {
                    tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Multi)));
                }
                '/' => {
                    tokens.push(Token::Operator(Operator::Arithmetic(ArithmeticOperator::Div)));
                }
                '=' => {
                    tokens.push(Token::Equal);
                }
                '{' => {
                    tokens.push(Token::OCurly);
                }
                '}' => {
                    tokens.push(Token::CCurly);
                }
                ';' => {
                    tokens.push(Token::Semi);
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


