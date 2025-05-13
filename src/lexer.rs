#[derive(Debug, Clone)]
pub enum Token {
    Let,
    Print,
    Spawn,
    Wait,
    Identifier(String),
    StringLiteral(String),
    Equals,
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Comma,
    Eof,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespaces
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }

            // Symbols
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Add);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Subtract);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Divide);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }

            // String literals
            '"' => {
                chars.next(); // skip opening quote
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        break;
                    }
                    value.push(c);
                    chars.next();
                }
                chars.next(); // skip closing quote
                tokens.push(Token::StringLiteral(value));
            }

            // Identifiers and keywords
            ch if ch.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "print" => tokens.push(Token::Print),
                    "spawn" => tokens.push(Token::Spawn),
                    "wait" => tokens.push(Token::Wait),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }

            // Skip any unrecognized character
            _ => {
                chars.next(); // Skip unknown character
            }
        }
    }

    tokens.push(Token::Eof); // End-of-file marker
    tokens
}
