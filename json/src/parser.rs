


use std::collections::HashMap;
use crate::lexer::{Lexer, LexerError, Token};
use crate::value::JsonValue::{self};

#[derive(Debug)]
pub enum ParseError {
    MissingOpenBracket,
    ParseNumberError,
    UnexpectedTokenError,
    UnexpectedEndError,
    InvalidJsonFormat
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
pub fn new(input: &str) -> Result<Self, LexerError> {
    let mut lexer = Lexer::new(input);
    match lexer.lex_all() {
        Ok(tokens) => {
            return Ok(Self {
                tokens: tokens,
                pos: 0
            })
        }
        Err(err) => return Err(err)
    }
    
}

fn current(&self) -> Option<&Token> {
    if self.pos >= self.tokens.len() {
        return  None;
    }

    Some(&self.tokens[self.pos])
}

fn advance(&mut self) {
    self.pos += 1;
}

fn expect(&mut self, expected: Token) -> Result<(), ParseError>{
    match self.current() {
        Some(token) if *token == expected => {
            self.advance();
            Ok(())
        }

        Some(_) => return Err(ParseError::UnexpectedTokenError),
        None => return Err(ParseError::UnexpectedEndError)
    }
}

fn parse_number(&mut self) -> Result<JsonValue, ParseError> {
    match self.current() {
        Some(Token::Number(number)) => {
            match number.parse::<f64>() {
                Ok(f) => {
                    self.advance();
                    Ok(JsonValue::Number(f))
                }

                Err(_) => Err(ParseError::ParseNumberError)
            }
        }
        _ => Err(ParseError::MissingOpenBracket)
    }
}

fn parse_string(&mut self) -> Result<JsonValue, ParseError> {
    match self.current() {
        Some(Token::String(s)) => {
            let value = s.to_string();
            self.advance();
            Ok(JsonValue::String(value))
        }
        _ => Err(ParseError::UnexpectedTokenError)
    }
}

fn parse_bool(&mut self) -> Result<JsonValue, ParseError> {
    match self.current() {
        Some(Token::Boolean(b)) => {
            let value = *b;
            self.advance();
            Ok(JsonValue::Bool(value))
        }
        _ => return Err(ParseError::UnexpectedTokenError)
    }
}

fn parse_null(&mut self) -> Result<JsonValue, ParseError> {
    match self.current() {
        Some(Token::Null) => {
            self.advance();
            Ok(JsonValue::Null)
        }
        _ => return Err(ParseError::UnexpectedTokenError)
    }
}

fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
    
    self.advance();

    let mut array = Vec::<JsonValue>::new();
    
    loop {
        match self.current() {
            Some(Token::Rbracket) => {
                self.advance();
                break;
            }
            Some(_) => {
                let value = self.parse_value()?;
                array.push(value);
            }
            None => return Err(ParseError::UnexpectedEndError)
        }

        match self.current() {
            Some(Token::Comma) => {
                self.advance();
            }
            Some(Token::Rbracket) => {
                self.advance();
                break;
            }
            _ => return Err(ParseError::UnexpectedTokenError)
        }
    }

    Ok(JsonValue::Array(array))
}

fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
    
    self.advance();
    let mut object = HashMap::new();
    
    loop {
        match self.current() {
            Some(Token::String(key)) => {
                let key = key.to_string();
                self.advance();
                self.expect(Token::Colon)?;
                let value = self.parse_value()?;

                object.insert(key, value);
            }
            Some(Token::Rbrace) => {
                self.advance();
                break;
            }
            _ => return Err(ParseError::UnexpectedTokenError)
        }

        match self.current() {
            Some(Token::Comma) => {
                self.advance();
            }
            Some(Token::Rbrace) => {
                self.advance();
                break;
            }
            _ => return Err(ParseError::UnexpectedTokenError)
        }
    }

    Ok(JsonValue::Object(object))
}

fn parse_value(&mut self) -> Result<JsonValue, ParseError> {

    match self.current() {
        Some(Token::String(_))    =>  self.parse_string(),
        Some(Token::Number(_))    =>  self.parse_number(),
        Some(Token::Lbrace)       =>  self.parse_object(),
        Some(Token::Lbracket)     =>  self.parse_array(),
        Some(Token::Boolean(_))   =>  self.parse_bool(),
        Some(Token::Null)         =>  self.parse_null(),
        None => Err(ParseError::UnexpectedTokenError),
        _ => return Err(ParseError::MissingOpenBracket)    
    }
}

pub fn parse(&mut self) -> Result<JsonValue, ParseError> {
    let value = self.parse_value()?;

    match self.current() {
        None => Ok(value),
        _ => Err(ParseError::UnexpectedEndError)
    }
}

}