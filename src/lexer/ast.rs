// Grammar for our language
//
// expression     → literal| unary | binary | grouping ;
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;

use crate::lexer::tokens::*;

mod ast {
    pub enum Expression {
        literal(Literal),
        unary(Unary),
        binary(Binary),
        grouping(Grouping)
    }

    pub enum Literal {
        Number(Literal::Integer),
        String(Literal::String),
        Boolean(Literal::Boolean),
        Nil(Token::NIL)
    }

    pub struct Grouping {
        expr: Expression,
    }

    pub struct Unary {
        op: Token::Unary,
        expr: Expression,
    }

    pub struct Binary {
        op: Token::Binary,
        lhs: Expression,
        rhs: Expression,
    }
}
