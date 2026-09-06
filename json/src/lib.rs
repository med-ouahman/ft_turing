mod lexer;
mod value;
mod parser;

pub use crate::lexer::{Lexer, LexerError, Token};
pub use crate::parser::Parser;

pub fn parse(input: &str) -> Result<Vec<Token>, LexerError> {
    Lexer::new(input).lex_all()
}


