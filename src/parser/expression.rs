// Grammar for our language
//
// expression     → literal| unary | binary | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;

use crate::lexer::tokens;
use crate::lexer::tokens::{Literal, Token, VALID_SYMBOLS};
use crate::parser::Parser;
use crate::Result;
use crate::{peek_ident, peek_literal, peek_symbol, trace};
use std::fmt;
use error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal {
        value: Literal,
    },
    Unary {
        op: Token, //unary symbol
        expr: Box<Expression>,
    },
    Binary {
        op: Token, // binary symbol
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
    Variable {
        name: String
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Literal {
//     Number(u32),
//     String(String),
//     Boolean(bool),
//     Nil(Token), // Check manually if Token::NIL is encountered
// }
//
// impl fmt::Display for Literal {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// Can make a visitor pattern when we have multiple data-types to work with
// and
impl Parser {
    pub fn parse_expr(&mut self) -> Result<Expression> {
        trace!("expr");
        let lhs = self.parse_expr_no_binary()?;
        self.parse_binary_r_expr(0, lhs)
    }

    pub fn parse_expr_no_binary(&mut self) -> Result<Expression> {
        // trace!("non-binary expr ");
        match self.tokens.peek() {
            Some(Token::LITERAL(_)) => self.parse_literal_expr(),
            Some(Token::IDENTIFIER(_)) => self.parse_identifier_expr(),
            Some(Token::LPAREN) => self.parse_grouping(),
            Some(Token::MINUS) | Some(Token::BANG) => self.parse_unary_expr(),
            _ => Err("Unable to parse expr".to_string()),
        }
    }

    pub fn parse_identifier_expr(&mut self) -> Result<Expression> {
        trace!("identifier expr");
        let name = peek_ident!(self);
        let expr = Ok(Expression::Variable{ name });
        self.tokens.next();
        expr
    }

    pub fn parse_literal_expr(&mut self) -> Result<Expression> {
        trace!("literal expr");
        let value = peek_literal!(self);
        let expr = Ok(Expression::Literal{ value });
        self.tokens.next();
        expr
    }

    pub fn parse_binary_r_expr(
        &mut self,
        op_precedence: i32,
        lhs: Expression,
    ) -> Result<Expression> {
        trace!("binary_r expr");
        let mut lhs = lhs;
        loop {
            let op = peek_symbol!(self);
            let curr_precedence = tokens::binary_op_precedence(&op);

            if curr_precedence < op_precedence {
                return Ok(lhs.clone());
            }
            self.tokens.next();

            let mut rhs = self.parse_expr_no_binary()?;

            let next_precedence = tokens::binary_op_precedence(&peek_symbol!(self));

            if op_precedence < next_precedence {
                rhs = self.parse_binary_r_expr(op_precedence + 1, rhs)?;
            }

            lhs = Expression::Binary {
                op: *VALID_SYMBOLS.get(op.as_str()).clone().okk_or_else(|| Err()),
                lhs: Box::new(lhs.clone()),
                rhs: Box::new(rhs),
            };
        }
    }

    pub fn parse_unary_expr(&mut self) -> Result<Expression> {
        trace!("unary expr");
        let op = VALID_SYMBOLS.get(peek_symbol!(self).as_str())?.clone();
        self.tokens.next();
        let expr = Box::new(self.parse_expr_no_binary()?);
        Ok(Expression::Unary { op, expr })
    }

    pub fn parse_grouping(&mut self) -> Result<Expression> {
        trace!("grouping");
        if !self.next_symbol("(") {
            return Err("misidentified grouping expression".to_string());
        }
        let expr = Box::new(self.parse_expr()?);
        if !self.next_symbol(")") {
            return Err("expected ')' after grouping expression".to_string());
        }
        Ok(Expression::Grouping { expr })
    }
}
