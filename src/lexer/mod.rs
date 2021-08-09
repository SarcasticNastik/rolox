pub mod tokens;

use crate::util;
use crate::Result;
use crate::lexer::tokens::*;
use std::iter::Peekable;
use std::vec::IntoIter;
use ::phf::Map;

#[macro_use]
macro_rules! trace {
    ($msg:literal) => {
        eprintln!("lexer: {}", $msg);
    };
    ($msg:literal,$val:expr) => {
        eprintln!("lexer: {}", format!($msg, $val));
    };
}

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
    line: u32 // somehow keep track of line number
}

impl Lexer {
    pub fn new(text: &String) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
            line: 0
        }
    }

    fn take_while(&mut self,
                  raw_token: &mut String,
                  cond: fn(char) -> bool
    ) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                },
                _ => {
                    // self.raw_data.peek();
                    trace!("Stopping get_next_char_while after peeking {:?}", self.raw_data.peek());
                    break;
                }
            }
        }
    }

    fn is_identifier(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}

impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Self::Item;
        let first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                    break;
                },
                None => return None,
            }
        }

        trace!("First character to lexer: {:?}", first_char);

        // IDENTIFIER
        if Self::is_identifier(first_char) && !first_char.is_numeric() {
            trace!("Idenifier");
            let mut name = first_char.to_string();
            self.take_while(&mut name, Self::is_identifier);

            // Check for reserved words
            token = match name.as_str() {
                "and" => Ok(Token::AND),
                "class" => Ok(Token::CLASS),
                "else" => Ok(Token::ELSE),
                "false" => Ok(Token::FALSE),
                "fun" => Ok(Token::FUN),
                "for" => Ok(Token::FOR),
                "if" => Ok(Token::IF),
                "nil" => Ok(Token::NIL),
                "or" => Ok(Token::OR),
                "print" => Ok(Token::PRINT),
                "return" => Ok(Token::RETURN),
                "super" => Ok(Token::SUPER),
                "this" => Ok(Token::THIS),
                "true" => Ok(Token::TRUE),
                "var" => Ok(Token::VAR),
                "while" => Ok(Token::WHILE),
                val => Ok(Token::IDENTIFIER(val.to_string()))
            }
        }
        // NUMBER - Maybe have decimal point?
        else if first_char.is_numeric() {
            trace!("Numeric");
            let mut value = first_char.to_string();
            self.take_while(&mut value, |c| c.is_numeric());
            token = match value.parse() {
                Ok(i) => Ok(Token::LITERAL(Literal::INTEGER(i))),
                Err(_) => Err(format!("Integer literal error: {}", value)),
            }
        }
        // String literal
        else if first_char == '"' {
            trace!("String literal");
            let mut value = String::new();
            self.take_while(&mut value, |c| c != '"');
            self.raw_data.next(); // eat the ending "
            token = Ok(Token::LITERAL(Literal::STRING(value)));

        }
        // Symbol
        else {
            trace!("Symbol");
            let mut raw = first_char.to_string();
            loop {
                if let Some(val) = self.raw_data.peek() {
                    raw.push(*val);
                }
                else {
                    break;
                }

                // Check if entry exists in the hashmap
                if VALID_SYMBOLS.contains_key(raw.as_str()) {
                    self.raw_data.next();
                }
                else {
                    raw.pop();
                    break;
                }
            }
            token = match &raw[..] {
                s if s == "//" => {
                    trace!("Ignoring comments!");
                    self.take_while(&mut String::new(), |c| c != '\n');
                    self.line += 1;
                    self.next()?
                }
                s if VALID_SYMBOLS.contains_key(s) => Ok(VALID_SYMBOLS.get(s).cloned()?),
                _ => Err(format!("Unknown token {}", raw)),
            }
        }
        Some(token)
    }
}

