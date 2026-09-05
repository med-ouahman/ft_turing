// lexer


enum Token {
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Comma,
    Colon,
    Quote,
    Number(String),
    String(String),
    Key,
    Value
}

enum TokenError {

}

fn next() -> Token {

}

pub fn lex(input: &str) -> Result<Vec<Token>, TokenError> {
    let tokens: Vec<Token>::new();
    
    for c in input.chars() {
        let token: Token = next();
        tokens.push(token)
    }
}
