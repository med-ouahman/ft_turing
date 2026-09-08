


use std::collections::HashMap;
use crate::lexer::{Lexer, LexerError, Token};
use crate::value::JsonValue::{self};

#[derive(Debug)]
pub enum ParseError {
    MissingOpenBracket,
    ParseNumberError,
    UnexpectedTokenError,
    UnexpectedEndError
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

fn parse_number(&mut self) -> Result<f64, ParseError> {
    match self.current() {
        Some(Token::Number(number)) => {
            match number.parse::<f64>() {
                Ok(f) => return  Ok(f),
                Err(_) => Err(ParseError::ParseNumberError)
            }
        }
        _ => Err(ParseError::MissingOpenBracket)
    }
}

fn parse_string(&mut self) -> Result<String, ParseError> {
    match self.current() {
        Some(Token::String(s)) => return Ok(s.to_string()),
        _ => Err(ParseError::UnexpectedTokenError)
    }
}

fn parse_bool(&mut self) -> Result<bool, ParseError> {
    Ok(false)
}

fn parse_null(&mut self) -> Result<(), ParseError> {
    Ok(())
}

fn parse_array(&mut self) -> Result<Vec<JsonValue>, ParseError> {
    self.advance(); // Skip the opening bracket
    let mut array = Vec::<JsonValue>::new();

    // loop until ']' | ParseError
    
    loop {
        match self.current() {
            Some(Token::Rbracket) => {
                self.advance(); // Skip the closing bracket
                break;
            }
            Some(_) => {
                let value = self.parse()?;
                array.push(value);
            }
            None => return Err(ParseError::UnexpectedEndError)
        }

        match self.current() {
            Some(Token::Comma) => {
                self.advance(); // Skip the comma
            }
            Some(Token::Rbracket) => {
                self.advance(); // Skip the closing bracket
                break;
            }
            _ => return Err(ParseError::UnexpectedTokenError)
        }
    }

    Ok(array)
}

fn parse_object(&mut self) -> Result<HashMap<String, JsonValue>, ParseError> {
    
    self.advance();
    let mut object = HashMap::new();
    
    loop {
        match self.current() {
            Some(Token::String(key)) => {
                let key = key.to_string();
                self.advance();
                self.expect(Token::Colon)?;
                let value = self.parse()?;

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

    Ok(object)
}

pub fn parse(&mut self) -> Result<JsonValue, ParseError> {

    for token in &self.tokens {
        println!("{:#?}", token);
    }

    loop {

        match self.current() {
            Some(token) => {
                match token {
                    Token::String(_) => {
                        match self.parse_string() {
                            Ok(s) => return Ok(JsonValue::String(s)),
                            Err(err) => return Err(err)
                        }
                    }
                    Token::Number(_) => {
                        match self.parse_number() {
                            Ok(n) => return Ok(JsonValue::Number(n)),
                            Err(e) => return Err(e)
                        }
                    }
                    Token::Lbrace => {
                        match self.parse_object() {
                            Ok(obj) => return Ok(JsonValue::Object(obj)),
                            Err(err) => return Err(err)
                        }
                    }
                    Token::Lbracket => {
                        match self.parse_array() {
                            Ok(arr) => return Ok(JsonValue::Array(arr)),
                            Err(err) => return Err(err)
                        }
                    }
                    Token::Boolean(_) => {
                        match self.parse_bool() {
                            Ok(b) => return Ok(JsonValue::Bool(b)),
                            Err(err) => return  Err(err)
                        }
                    }
                    Token::Null => {
                        match self.parse_null() {
                            Ok(_) => return Ok(JsonValue::Null),
                            Err(err) => return Err(err)
                        }
                    }
                    _ => return Err(ParseError::MissingOpenBracket)
                    
                };
            }
            None => break
        }
        
    }

    Ok(JsonValue::Bool(true))
}

}
