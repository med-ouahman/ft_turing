mod lexer;
mod value;
mod parser;


pub use crate::lexer::{Lexer, LexerError, Token};
pub use crate::parser::Parser;

pub use crate::value::JsonValue;
pub use crate::parser::ParseError;

pub fn parse(input: &str) -> Result<JsonValue, ParseError> {

    let mut parser = match Parser::new(input) {
        Ok(parser) => parser,
        Err(_) => return Err(ParseError::UnexpectedTokenError)
    };

    parser.parse()
}


