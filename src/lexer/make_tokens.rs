use std::iter::Peekable;
use std::str::Chars;

// Token struct
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Pipe, Caret, Ampersand, Percent,    // | ^ & %
    
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
    Print, Return, Super, This, True, Var, While,

    // Types
    Int, Float, Bool, Void,
    Int8, Int16, Int32, Int64,
    Uint, Uint8, Uint16, Uint32, Uint64,
    Float32, Float64,
    Byte, Rune, Char,   // Byte is an alias for Uint8, Rune is an alias for Int32, Char is an alias for Rune
    
    // End of file
    Eof
}

#[derive(Debug, PartialEq)]
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
            column: 1
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
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line
                        while self.peek() != '\n' && self.peek() != '\0' {
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
                        break;
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
        // Skip whitespace
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
            '-' => self.make_token(TokenType::Minus, "-".to_string()),
            '+' => self.make_token(TokenType::Plus, "+".to_string()),
            ';' => self.make_token(TokenType::Semicolon, ";".to_string()),
            '/' => self.make_token(TokenType::Slash, "/".to_string()),
            '|' => self.make_token(TokenType::Pipe, "|".to_string()),
            '^' => self.make_token(TokenType::Caret, "^".to_string()),
            '&' => self.make_token(TokenType::Ampersand, "&".to_string()),
            '%' => self.make_token(TokenType::Percent, "%".to_string()),
            // One or two character tokens
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
            '*' => {
                if self.match_char('*') {
                    self.make_token(TokenType::StarStar, "**".to_string())
                } else {
                    self.make_token(TokenType::Star, "*".to_string())
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
