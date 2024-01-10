use std::iter::Peekable;
use std::str::Chars;


// Token struct
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Pipe, Caret, Ampersand, Percent, Colon,    // | ^ & %
    
    // One or two character tokens
    Bang, BangEqual,    // ! !=
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    StarStar, PipePipe, // ** ||
    AmpersandAmpersand, // &&
    SlashSlash, StarSlash, // // /*
    
    // Literals
    Identifier, String, Number,
    
    // Keywords
    And, Class, Else, False, Func, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, Const,

    // Types
    Int, Float, Bool, Void,
    Int8, Int16, Int32, Int64,
    Uint, Uint8, Uint16, Uint32, Uint64,
    Float32, Float64,
    Byte, Rune, Char,   // Byte is an alias for Uint8, Rune is an alias for Int32, Char is an alias for Rune
    
    // End of file
    Eof
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
            column
        }
    }
}

// Lexer struct
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekable(),
            line: 1,
            column: 0
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.get_token();
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    pub fn make_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line, self.column)
    }

    pub fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::Eof, message.to_string(), self.line, self.column)
    }

    pub fn advance(&mut self) -> Option<char> {
        self.source.next().map(|c| {
            self.column += 1;
            c
        })
    }

    pub fn peek(&mut self) -> char {
        *self.source.peek().unwrap_or(&'\0')
    }
    
    pub fn peek_next(&mut self) -> char {
        self.source.clone().next().unwrap_or('\0')
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                },
                '/' => {
                    self.advance(); // Consume the '/' character that was peeked
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line
                        while self.peek() != '\n' && self.peek() != '\0' {
                            self.advance();
                        }
                    } else if self.peek_next() == '*' {
                        // The end of a block comment is denoted by "*/"
                        self.advance();
                        while self.peek() != '\0' {
                            if self.peek() == '*' && self.advance().is_some() && self.peek_next() == '/' {
                                self.advance();
                                break;
                            } else if self.peek() == '\n' {
                                self.line += 1;
                                self.column = 1;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
    }

    pub fn string(&mut self) -> Token {
        let mut string = String::new();
        loop {
            let c = self.advance();
            if c == Some('"') || c == Some('\''){
                break;
            } else if c == None {
                return self.error_token("Unterminated string");
            } else if c == Some('\n') {
                self.line += 1;
                self.column = 0;
            }
            string.push(c.unwrap());
        }
        Token::new(TokenType::String, string, self.line, self.column)
    }
    
    pub fn number(&mut self, current_char: char) -> Token {
        let mut number = String::new();
        loop {
            let c = self.peek().to_string();
            // First character is already checked in get_token() so add it to the number
            if number.len() == 0 {
                number.push(current_char);
            }
            match c.chars().next().unwrap() {
                '0'..='9' => {
                    number.push(self.advance().unwrap());
                },
                '.' => {
                    number.push(self.advance().unwrap());   // Consume the '.' character that was peeked
                    if self.peek_next().is_digit(10) {
                        number.push(self.advance().unwrap());
                    } else {
                        // If the next character is not a digit, then the '.' is not part of the number
                        // So break the loop and raise an error
                        let error_literal: String = self.peek_next().to_string();
                        return self.error_token(format!("Invalid literal '{}' found, expected a digit", &error_literal).as_str());
                    }
                },
                'e' | 'E' => {
                    number.push(self.advance().unwrap());   // Consume the 'e' or 'E' character that was peeked
                    if self.peek_next() == '-' || self.peek_next() == '+' {
                        number.push(self.advance().unwrap());
                        loop {
                            let next_char = self.peek_next();
                            if next_char.is_digit(10) {
                                number.push(self.advance().unwrap());
                            } else if next_char == ' ' || next_char == ';' || next_char == ')' || next_char == '}' || next_char == ']' {
                                break;  // Break the loop if the next character is a whitespace or a delimiter
                            } else {
                                let err_msg = format!("Invalid literal '{}' found, expected a digit", self.peek_next());
                                return self.error_token(err_msg.as_str());
                            }
                        }
                    } else if self.peek_next().is_digit(10) {
                        number.push(self.advance().unwrap());
                    } else {
                        let err_msg = format!("Invalid literal '{}' found, expected a digit", self.peek_next());
                        return self.error_token(err_msg.as_str());
                    }
                },
                _ => {
                    break;
                }
            }
        }
        Token::new(TokenType::Number, number, self.line, self.column)
    }    

    pub fn identifier(&mut self, current_char: char) -> Token {
        let mut identifier = String::new();
        loop {
            let c = self.peek();
            // First character is already checked in get_token() so add it to the identifier
            if identifier.len() == 0 {
                identifier.push(current_char);
            }
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    identifier.push(self.advance().unwrap());
                },
                _ => {
                    break;
                }
            }
        }
        // Check if the identifier is a keyword
        let token_type = match identifier.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "False" => TokenType::False,
            "func" => TokenType::Func,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "True" => TokenType::True,
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "while" => TokenType::While,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "void" => TokenType::Void,
            "int8" => TokenType::Int8,
            "int16" => TokenType::Int16,
            "int32" => TokenType::Int32,
            "int64" => TokenType::Int64,
            "uint" => TokenType::Uint,
            "uint8" => TokenType::Uint8,
            "uint16" => TokenType::Uint16,
            "uint32" => TokenType::Uint32,
            "uint64" => TokenType::Uint64,
            "float32" => TokenType::Float32,
            "float64" => TokenType::Float64,
            "byte" => TokenType::Byte,
            "rune" => TokenType::Rune,
            "char" => TokenType::Char,
            _ => TokenType::Identifier
        };
        Token::new(token_type, identifier, self.line, self.column)
    }

    pub fn get_token(&mut self) -> Token {
        // Skip whitespace and comments
        self.skip_whitespace();
    
        // Get the next character
        let c = match self.advance() {
            Some(ch) => ch,
            None => return self.make_token(TokenType::Eof, "".to_string())
        };
    
        // Match the character
        match c {
            // Single-character tokens
            '(' => self.make_token(TokenType::LeftParen, "(".to_string()),
            ')' => self.make_token(TokenType::RightParen, ")".to_string()),
            '{' => self.make_token(TokenType::LeftBrace, "{".to_string()),
            '}' => self.make_token(TokenType::RightBrace, "}".to_string()),
            ',' => self.make_token(TokenType::Comma, ",".to_string()),
            '.' => self.make_token(TokenType::Dot, ".".to_string()),
            '+' => self.make_token(TokenType::Plus, "+".to_string()),
            ';' => self.make_token(TokenType::Semicolon, ";".to_string()),
            '^' => self.make_token(TokenType::Caret, "^".to_string()),
            '%' => self.make_token(TokenType::Percent, "%".to_string()),
            ':' => self.make_token(TokenType::Colon, ":".to_string()),

            // One or two character tokens
            '-' => {
                // might be a negative number
                if self.peek().is_digit(10) {
                   self.number(c)
                } else {
                    self.make_token(TokenType::Minus, "-".to_string())
                }
            },
            '*' => {
                if self.match_char('*') {
                    self.make_token(TokenType::StarStar, "**".to_string())
                } else {
                    self.make_token(TokenType::Star, "*".to_string())
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual, "!=".to_string())
                } else {
                    self.make_token(TokenType::Bang, "!".to_string())
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual, "==".to_string())
                } else {
                    self.make_token(TokenType::Equal, "=".to_string())
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual, ">=".to_string())
                } else {
                    self.make_token(TokenType::Greater, ">".to_string())
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual, "<=".to_string())
                } else {
                    self.make_token(TokenType::Less, "<".to_string())
                }
            },
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenType::PipePipe, "||".to_string())
                } else {
                    self.make_token(TokenType::Pipe, "|".to_string())
                }
            },
            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenType::AmpersandAmpersand, "&&".to_string())
                } else {
                    self.make_token(TokenType::Ampersand, "&".to_string())
                }
            },
            '/' => {
                if self.match_char('/') {
                    self.make_token(TokenType::SlashSlash, "//".to_string())
                } else if self.match_char('*') {
                    self.make_token(TokenType::StarSlash, "/*".to_string())
                } else {
                    self.make_token(TokenType::Slash, "/".to_string())
                }
            },
            
            // Literals
            '"' | '\'' => self.string(),
            '0'..='9' => self.number(c),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),
            
            // End of file
            '\0' => self.make_token(TokenType::Eof, self.column.to_string()),
            
            // Invalid character
            _ => self.error_token(format!("Unexpected character found: '{}'", c).as_str())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        let mut lexer = Lexer::new("1 + 2");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 5));
    }

    #[test]
    fn test_get_tokens() {
        let mut lexer = Lexer::new("1 + 2");
        assert_eq!(lexer.get_tokens(), vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 3),
            Token::new(TokenType::Number, "2".to_string(), 1, 5),
            Token::new(TokenType::Eof, "".to_string(), 1, 5)
        ]);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("1 + 2");
        lexer.skip_whitespace();
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 5));
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"Hello World\"");
        assert_eq!(lexer.get_token(), Token::new(TokenType::String, "Hello World".to_string(), 1, 13));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 13));
    }

    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "123".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 3));
    }

    #[test]
    fn number_with_exponent() {
        let mut lexer = Lexer::new("123e4");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "123e4".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 5));
    }

    #[test]
    fn number_with_exponent_and_sign() {
        let mut lexer = Lexer::new("123e+412; 123e-43; 123e41; 123E4144; 123E+413");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "123e+412".to_string(), 1, 8));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Semicolon, ";".to_string(), 1, 9));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "123e-43".to_string(), 1, 17));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Semicolon, ";".to_string(), 1, 18));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "123e41".to_string(), 1, 25));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Semicolon, ";".to_string(), 1, 26));
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("abc");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Identifier, "abc".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 3));
    }

    #[test]
    fn test_get_token_with_whitespace() {
        let mut lexer = Lexer::new("1 + 2");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 5));
    }

    #[test]
    fn test_get_token_with_comments() {
        let mut lexer = Lexer::new("1 + 2 // This is a comment");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 26));
    }

    #[test]
    fn test_get_token_with_block_comment() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment */");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 1, 35));
    }

    #[test]
    fn test_get_token_with_block_comment_on_multiple_lines() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment\nThis is a block comment\nThis is a block comment */");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 3, 28));
    }

    #[test]
    fn test_get_token_with_block_comment_on_multiple_lines_and_whitespace() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment\nThis is a block comment\nThis is a block comment */");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 3, 28));
    }

    #[test]
    fn test_get_token_with_block_comment_on_multiple_lines_and_whitespace_and_code() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment\nThis is a block comment\nThis is a block comment */ 3 + 4");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "3".to_string(), 3, 30));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 3, 32));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "4".to_string(), 3, 34));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 3, 34));
    }

    #[test]
    fn test_get_token_with_block_comment_on_multiple_lines_and_whitespace_and_code_and_comments() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment\nThis is a block comment\nThis is a block comment */ 3 + 4 // This is a comment");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "3".to_string(), 3, 30));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 3, 32));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "4".to_string(), 3, 34));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 3, 55));
    }

    #[test]
    fn test_get_token_with_block_comment_on_multiple_lines_and_whitespace_and_code_and_comments_and_code() {
        let mut lexer = Lexer::new("1 + 2 /* This is a block comment\nThis is a block comment\nThis is a block comment */ 3 + 4 // This is a comment\n5 + 6");
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "1".to_string(), 1, 1));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 1, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "2".to_string(), 1, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "3".to_string(), 3, 30));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 3, 32));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "4".to_string(), 3, 34));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "5".to_string(), 4, 3));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Plus, "+".to_string(), 4, 5));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Number, "6".to_string(), 4, 7));
        assert_eq!(lexer.get_token(), Token::new(TokenType::Eof, "".to_string(), 4, 7));
    }
}
