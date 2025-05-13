#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    Func,
    Spawn,
    Wait,
    Identifier(String),
    StringLiteral(String),
    Equals,
    Devide,
    Multiply,
    Subtract,
    Add,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,
    LeftParen,
    RightParen,
    Comma,
    Comment(String),
    Eof,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            // Skip whitespace
            ' ' | '\n' | '\r' | '\t' => continue,

            // Handle comments
            '/' => {
                // Check if the next character is also a '/'
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        chars.next(); // Consume the second '/'
                        let mut comment = String::new();
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch == '\n' {
                                break;
                            }
                            comment.push(next_ch);
                            chars.next(); // Consume the character
                        }
                        tokens.push(Token::Comment(comment));
                    } else {
                        tokens.push(Token::Devide);
                    }
                } else {
                    tokens.push(Token::Devide);
                }
            }

            // Handle symbols
            '=' => tokens.push(Token::Equals),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Subtract),
            '*' => tokens.push(Token::Multiply),
            '<' => {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        chars.next(); // Consume the '='
                        tokens.push(Token::LessThanOrEqual);
                    } else {
                        tokens.push(Token::LessThan);
                    }
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        chars.next(); // Consume the '='
                        tokens.push(Token::GreaterThanOrEqual);
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '!' => {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        chars.next(); // Consume the '='
                        tokens.push(Token::NotEqual);
                    } else {
                        eprintln!("Unexpected character: {}", ch);
                    }
                } else {
                    eprintln!("Unexpected character: {}", ch);
                }
            }

            // Handle string literals
            '"' => {
                let mut string = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' {
                        chars.next(); // Consume the closing quote
                        break;
                    }
                    string.push(next_ch);
                    chars.next(); // Consume the character
                }
                tokens.push(Token::StringLiteral(string));
            }

            // Handle integer and float literals
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);
                let mut is_float = false;

                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '.' {
                        is_float = true;
                        number.push(next_ch);
                        chars.next(); // Consume the '.'
                    } else if next_ch.is_digit(10) {
                        number.push(next_ch);
                        chars.next(); // Consume the digit
                    } else {
                        break;
                    }
                }

                if is_float {
                    if let Ok(float_value) = number.parse::<f64>() {
                        tokens.push(Token::FloatLiteral(float_value));
                    }
                } else {
                    if let Ok(int_value) = number.parse::<i64>() {
                        tokens.push(Token::IntegerLiteral(int_value));
                    }
                }
            }

            // Keywords and identifiers
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut identifier = String::new();
                identifier.push(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        identifier.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "let" => tokens.push(Token::Let),
                    "print" => tokens.push(Token::Print),
                    "func" => tokens.push(Token::Func),
                    "spawn" => tokens.push(Token::Spawn),
                    "wait" => tokens.push(Token::Wait),
                    _ => {
                        if identifier == "True" {
                            tokens.push(Token::BooleanLiteral(true));
                        } else if identifier == "False" {
                            tokens.push(Token::BooleanLiteral(false));
                        } else {
                            // Its not a keyword, so it must be an identifier
                            tokens.push(Token::Identifier(identifier));
                        }
                    }
                }
            }

            // Catch any unexpected characters (error handling)
            _ => {
                eprintln!("Unexpected character: {}", ch);
                continue;
            }
        }
    }

    tokens.push(Token::Eof); // Mark the end of the tokens
    tokens
}
