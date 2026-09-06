/* Json Lexer */

#[derive(Debug)]
pub enum Token {
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
pub enum LexerError {
	UnexpectedToken,
	InvalidCharacter,
	UnterminatedString,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize
}

impl <'a> Lexer<'a> {

pub fn new(input: &'a str) -> Self {
    Self { input, pos: 0 }
}

fn current_char(&self) -> Option<char> {
	self.input[self.pos..].chars().next()
}

fn is_number_start(&self, c: char) -> bool {
	c == '-' || c.is_ascii_digit()
}

fn scan_string(&mut self) -> Result<Token, LexerError> {
	
	let mut string = String::new();

	let mut escaped = false;

	while let Some(c) = self.current_char() {
		self.pos += c.len_utf8();
		if escaped {
			escaped = false;
			string.push(c);
			continue;
		}
		match c {
			'"' => return Ok(Token::String(string)),
			'\\' => escaped = true,
			_ => string.push(c)
		}
	}

	Err(LexerError::UnterminatedString)
}

fn is_token_end(c: char) -> bool {
	c.is_whitespace()
		|| c == ','
		|| c == ']'
		|| c == '}'
}

fn scan_exponent(&mut self) -> Result<String, LexerError> {
	let mut expo = String::new();

	if let Some(c) = self.current_char() {
		if c == 'e' || c == 'E' {
			expo.push(c);
			self.pos += 1;
		}
	}

	let curr = self.current_char();

	if let Some('-' | '+') = curr {
		expo.push(curr.unwrap());
		self.pos += 1;
	} else {
		return Err(LexerError::UnexpectedToken);
	}

	let mut has_digit = false;
	while let Some(c) = self.current_char() {
		match c {
			_ => {
				if Self::is_token_end(c) {
					return if has_digit {Ok(expo)} else {Err(LexerError::InvalidCharacter)};
				}
				self.pos += c.len_utf8();
				if c.is_ascii_digit() {
					has_digit = true;
					expo.push(c);
				} else {
					return Err(LexerError::UnexpectedToken);
				}
			}
		}
	}

	return if has_digit { Ok(expo) } else { Err(LexerError::UnexpectedToken)};
}

fn scan_fraction(&mut self) -> Result<String, LexerError> {
	
	let mut fraction = String::new();
	let curr = self.current_char();
	if let Some('.') = curr {
		fraction.push(curr.unwrap());
		self.pos += 1;
	}

	let mut has_digit = false;

	while let Some(c) = self.current_char() {
		match c {

			'e' | 'E' => {
				match self.scan_exponent() {
					Ok(exp) => {
						fraction.push_str(&exp);
					}
					Err(err) => return Err(err)
				}
			}
			_ => {
				if Self::is_token_end(c) {
					return if has_digit {Ok(fraction)} else {Err(LexerError::InvalidCharacter)};
				}
				if c.is_ascii_digit() {
					self.pos += c.len_utf8();
					has_digit = true;
					fraction.push(c);
				} else {
					return Err(LexerError::UnexpectedToken);
				}
			}
		}
	}

	return if has_digit {Ok(fraction)} else {Err(LexerError::InvalidCharacter)};
}


fn scan_number(&mut self) -> Result<Token, LexerError> {

	let mut number = String::new();

	if  Some('-') == self.current_char() {
		self.pos += '-'.len_utf8();
		number.push('-');
	}

	let mut has_digit = false;
	while let Some(c) = self.current_char() {
		match c {
			'e' | 'E' => {
				match self.scan_exponent() {
					Ok(string) => number.push_str(&string),
					Err(err) => return Err(err)
				}
			}
			'.' => {
				match self.scan_fraction() {
					Ok(fraction) => number.push_str(&fraction),
					Err(err) => return Err(err)
				}
			}
			_ => {
				if Self::is_token_end(c) {
					return if has_digit {Ok(Token::Number(number))} else {Err(LexerError::InvalidCharacter)};
				}

				self.pos += 1;
				if c.is_ascii_digit() {
					has_digit = true;
					number.push(c);
				} else {
					return Err(LexerError::UnexpectedToken);
				}
			}
		}
	}

	Ok(Token::Number(number))
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

			_ => {
				// Unsafe!!!!!!!
				self.pos -= c.len_utf8();
				if self.is_number_start(c) {
					match self.scan_number() {
						Ok(number) => return Ok(Some(number)),
						Err(err) => return Err(err)
					}
				}

				if c.is_whitespace() {
					self.pos += c.len_utf8();
				} else {	
					return Err(LexerError::UnexpectedToken);
				}
			}
		}
	}
	Ok(None)
}

pub fn lex_all(&mut self) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    loop {
        match self.next() {
            Ok(Some(token)) => tokens.push(token),
            Ok(None) => break,
            Err(err) => return Err(err),
        }
    }

    Ok(tokens)
}

}
