use std::collections::HashMap;
use ::phf::{Map, phf_map};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS, // either as a ~~unary operator~~ or binary operator
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    IDENTIFIER(String),
    LITERAL(Literal),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

// pub const KEYWORD: &[&str] = &["and", "class", "else", "false", "true", "fun", "for", "if", "Nil", "or", "print", "return", "super", "this", "var", "while"];

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    // Literals.
    STRING(String),
    INTEGER(i32),
    BOOLEAN(bool), // Do I really need this?
}

// Define precedence here

pub fn binary_op_precedence(op: &str) -> i32 {
    match op {
        "=" => 0,
        "==" | "!=" | "<" | ">" | "<=" | ">=" => 10,
        "+" | "-" => 20,
        "*" | "/" => 30,
        _ => -1
    }
}

pub static VALID_SYMBOLS: Map<&str, Token> = phf_map! {
    "(" => Token::LBRACE,
    ")" => Token::RBRACE,
    "{" => Token::LPAREN,
    "}" => Token::RPAREN,
    "," => Token::COMMA,
    "." => Token::DOT,
    "-" => Token::MINUS,
    "+" => Token::PLUS,
    ";" => Token::SEMICOLON,
    "/" => Token::SLASH,
    "*" => Token::STAR,
    "!" => Token::BANG,
    "!=" => Token::BANG_EQUAL,
    "=" => Token::EQUAL,
    "==" => Token::EQUAL_EQUAL,
    ">" => Token::GREATER,
    ">=" => Token::GREATER_EQUAL,
    "<" => Token::LESS,
    "<=" => Token::LESS_EQUAL,
};