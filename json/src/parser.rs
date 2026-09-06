

use crate::lexer::{Lexer, LexerError, Token};

use crate::value::Value::{self, Object};
use crate::value::{Value::Null};

enum ParseError {
    MissingOpenBracket,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            current: None,
        }
    }

    fn parse_object() -> Result<Object, ParseError> {
        Ok(Object(()))
    }
    fn parse_value() -> Result<Value, ParseError> {
        Ok(Value)
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        self.lexer.lex_all()
    }
}
