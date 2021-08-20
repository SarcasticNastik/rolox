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
            Some(Token::IDENTIFIER(name)) => name,
            _ => Err("Expected identifier".to_string()),
        };
    };
}

#[macro_export]
macro_rules! peek_literal {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::LITERAL(value)) => value.clone(),
            _ => Err("Expected literal".to_string()),
        };
    };
}

// Manually verify :')
#[macro_export]
macro_rules! peek_symbol {
    ($self:ident) => {
        match $self.tokens.peek() {
            Token::LBRACE => "(",
            Token::RBRACE => ")",
            Token::LPAREN => "{",
            Token::RPAREN => "}",
            Token::COMMA => ",",
            Token::DOT => ".",
            Token::MINUS => "-",
            Token::PLUS => "+",
            Token::SEMICOLON => ";",
            Token::SLASH => "/",
            Token::NOOP => "//",
            Token::STAR => "*",
            Token::BANG => "!",
            Token::BANG_EQUAL => "!=",
            Token::EQUAL => "=",
            Token::EQUAL_EQUAL => "==",
            Token::GREATER => ">",
            Token::GREATER_EQUAL => ">=",
            Token::LESS => "<",
            Token::LESS_EQUAL => "<=",
            _ => Err("Expected a valid symbol".to_string()),
        }
    };
}
