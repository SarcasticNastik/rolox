// Grammar for our language
//
// expression     → literal| unary | binary | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;

use crate::lexer::{Token, Literal as OtherLiteral};
use std::fmt;

#[derive(Debug, Clone,  PartialEq, Eq)]
pub enum Expression {
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone,  PartialEq, Eq)]
pub enum Literal {
    Number(u32),
    String(String),
    Boolean(bool),
    Nil, // Check manually if Token::NIL is encountered
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone,  PartialEq, Eq)]
pub struct Grouping {
    expr: Box<Expression>,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.expr)
    }
}

#[derive(Debug, Clone,  PartialEq, Eq)]
pub struct Unary {
    op: Token,
    expr: Box<Expression>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.op, self.expr)
    }
}

#[derive(Debug, Clone,  PartialEq, Eq)]
pub struct Binary {
    op: Token,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {}", self.lhs, self.op, self.rhs)
    }
}
