
#[derive(Debug)]
enum Token {
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Comma,
    Colon,
    Number(String),
    String(String),
}

#[derive(Debug)]
enum LexerError {
	InvalidCharacter,
	UnterminatedString
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl <'a> Lexer<'a> {
	fn scan_string(&mut self) -> Result<Token, LexerError> {
		
		let mut string = String::new();

		let chars = &mut self.input[self.pos..].char_indices();

		while let Some((_, c)) = &mut chars.next() {
			self.pos += c.len_utf8();

			match c {
				'"' => return Ok(Token::String(string)),
				_ => string.push(*c)
			}
		}

		Err(LexerError::UnterminatedString)
	}

	fn next(&mut self) -> Result<Option<Token>, LexerError> {

		if self.pos >= self.input.len() {
			return  Ok(None);
		}

		let current = &self.input[self.pos..];
		let mut chars = current.char_indices();

		while let Some((_, c)) = chars.next() {
			self.pos += c.len_utf8();
			match c {
				'{' => return Ok(Some(Token::Lbrace)),
				'}' => return Ok(Some(Token::Rbrace)),
				'[' => return Ok(Some(Token::Lbracket)),
				']' => return Ok(Some(Token::Rbracket)),
				',' => return Ok(Some(Token::Comma)),
				':' => return Ok(Some(Token::Colon)),
				'"' => {
					match self.scan_string() {
						Ok(token) => return Ok(Some(token)),
						Err(err) => return Err(err)
					}
				}
					
				_ => return Err(InvalidCharacter)
			}
		}
		Err(InvalidCharacter)
	}

}

fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
	let mut tokens: Vec<Token> = Vec::new();

	let mut lexer: Lexer = Lexer { input, pos: 0 };
	
	loop {
		match lexer.next() {
			Ok(Some(token)) => {
				tokens.push(token);
			}

			Ok(None) => {
				break;
			}

			Err(error) => {
				return  Err(error);
			}
		}
	}

	Ok(tokens)
}



use std::io;

use crate::{LexerError::InvalidCharacter, Token::{Colon, Comma, Lbrace, Lbracket, Rbrace, Rbracket}};

fn main() {
	loop {
		let mut input = String::new();
		io::stdin().read_line(& mut input).unwrap();
		
		let input = &input[0..input.len()-1];

		let tokens = lex(input).unwrap();
		
		for token in tokens {
			println!("{:?}", token);
		}
	}

}
