// Grammar for our language
//
// expression     → literal| unary | binary | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;

use crate::lexer::tokens::*;
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping)
}

impl fmt::Debug for Expression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {

    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
    Number(Literal::INTEGER),
    String(Literal::STRING),
    Boolean(Literal::BOOLEAN),
    Nil(Token::NIL),
}

#[derive(Debug, Clone, Hash)]
pub struct Grouping {
    expr: Expression,
}

#[derive(Debug, Clone, Hash)]
pub struct Unary {
    op: Token::Unary,
    expr: Expression,
}

#[derive(Debug, Clone, Hash)]
pub struct Binary {
    op: Token::Binary,
    lhs: Expression,
    rhs: Expression,
}

