// Refer to this for writing down the parser: [Temp Compiler](https://github.com/akmin04/yot-lang/)

mod expression;

use crate::lexer::tokens::{Token, VALID_SYMBOLS};
use std::{iter::Peekable, vec::IntoIter};

// Parser structure to iterate over the `lexed` tokens.
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<Token>>) -> Parser {
        Parser { tokens }
    }

    // if next symbol is `symbol`
    fn next_symbol(&mut self, symbol: &str) -> bool {
        match self.tokens.peek() {
            Some(s) => match s {
                Token::LBRACE
                | Token::RBRACE
                | Token::LPAREN
                | Token::RPAREN
                | Token::COMMA
                | Token::DOT
                | Token::MINUS
                | Token::PLUS
                | Token::SEMICOLON
                | Token::SLASH
                | Token::NOOP
                | Token::STAR
                | Token::BANG
                | Token::BANG_EQUAL
                | Token::EQUAL
                | Token::EQUAL_EQUAL
                | Token::GREATER
                | Token::GREATER_EQUAL
                | Token::LESS
                | Token::LESS_EQUAL => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! peek_ident {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::IDENTIFIER(name)) => String::from(name),
            _ => return Err("Expected identifier".to_string()),
        };
    };
}

#[macro_export]
macro_rules! peek_literal {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::LITERAL(value)) => value.clone(),
            _ => return Err("Expected literal".to_string()),
        };
    };
}

// Manually verify :')
#[macro_export]
macro_rules! peek_symbol {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::LBRACE) => "(".to_string(),
            Some(Token::RBRACE) => ")".to_string(),
            Some(Token::LPAREN) => "{".to_string(),
            Some(Token::RPAREN) => "}".to_string(),
            Some(Token::COMMA) => ",".to_string(),
            Some(Token::DOT) => ".".to_string(),
            Some(Token::MINUS) => "-".to_string(),
            Some(Token::PLUS) => "+".to_string(),
            Some(Token::SEMICOLON) => ";".to_string(),
            Some(Token::SLASH) => "/".to_string(),
            Some(Token::NOOP) => "//".to_string(),
            Some(Token::STAR) => "*".to_string(),
            Some(Token::BANG) => "!".to_string(),
            Some(Token::BANG_EQUAL) => "!=".to_string(),
            Some(Token::EQUAL) => "=".to_string(),
            Some(Token::EQUAL_EQUAL) => "==".to_string(),
            Some(Token::GREATER) => ">".to_string(),
            Some(Token::GREATER_EQUAL) => ">=".to_string(),
            Some(Token::LESS) => "<".to_string(),
            Some(Token::LESS_EQUAL) => "<=".to_string(),
            _ => return Err("Expected a valid symbol".to_string()),
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($msg:literal) => {
        eprintln!("parser: {}", $msg);
    };
    ($msg:literal,$val:expr) => {
        eprintln!("parser: {}", format!($msg, $val));
    };
}
