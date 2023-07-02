use crate::ast::{Operator, KeyWord, ArithmeticOperator};

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: char,
    src: Vec<char>,
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Operator(Operator),
    Assignment,
    Keyword(KeyWord),
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
        println!("Length of src: {}", src.len());
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
                return Some(Token::Keyword(KeyWord::Let));
            }
            "if" => {
                return Some(Token::Keyword(KeyWord::If));
            }
            "else" => {
                return Some(Token::Keyword(KeyWord::Else));
            }
            "for" => {
                return Some(Token::Keyword(KeyWord::For));
            }
            "while" => {
                return Some(Token::Keyword(KeyWord::While));
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

                println!("identifier is {:?}", identifier);
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
                println!("num is {:?}", num);
                return Token::Number(num);
            }
            self.position += 1;
        }
        panic!();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        println!("Position: {}, ", self.position);

        let mut tokens:Vec<Token> = Vec::new();
        while self.position < self.src.len() {
            self.ch = self.src[self.position];
            println!("Token: {}", self.ch);
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
                    tokens.push(Token::Operator(Operator::Equals));
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
                        println!("token is alphabetic");
                        tokens.push(self.read_identifier());
                    }
                    else if self.ch.is_numeric() {
                        println!("token is numeric");
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


