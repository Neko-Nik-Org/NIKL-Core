#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Diclaration keywords
    Let,
    Const,
    Function,
    Spawn,
    Wait,
    Assign,
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),

    // Data types
    Integer,
    Float,
    String,
    Boolean,
    Array,
    Tuple,
    HashMap,

    // Control flow keywords
    If,
    ElseIf,
    Else,
    And,
    Or,
    Not,
    Return,

    // Operators
    Equals,
    Divide,
    Multiply,
    Subtract,
    Add,

    // Comparison operators
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,

    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Arrow,

    // Keywords
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(char, usize, usize),
    UnterminatedString(usize, usize),
    InvalidNumber(String, usize, usize),
}


pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    column: usize,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        let next = self.chars.next();
        if let Some((_, c)) = next {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        next
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some((idx, ch)) = self.chars.peek().copied() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                }

                // Comments
                '/' => {
                    self.advance();
                    if let Some((_, '/')) = self.chars.peek().copied() {
                        self.advance();
                        while let Some((_, ch)) = self.chars.peek().copied() {
                            if ch == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Divide,
                            line: self.line,
                            column: self.column,
                        });
                    }
                }

                // Symbols
                '=' => {
                    self.advance();
                    if let Some((_, '=')) = self.chars.peek().copied() {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::Equals,
                            line: self.line,
                            column: self.column,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Assign,
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                '(' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        line: self.line,
                        column: self.column,
                    });
                }
                ')' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        line: self.line,
                        column: self.column,
                    });
                }
                '{' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftBrace,
                        line: self.line,
                        column: self.column,
                    });
                }
                '}' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightBrace,
                        line: self.line,
                        column: self.column,
                    });
                }
                '[' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftBracket,
                        line: self.line,
                        column: self.column,
                    });
                }
                ']' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightBracket,
                        line: self.line,
                        column: self.column,
                    });
                }
                ',' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        line: self.line,
                        column: self.column,
                    });
                }
                '+' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Add,
                        line: self.line,
                        column: self.column,
                    });
                }
                '-' => {
                    self.advance();
                    if let Some((_, '>')) = self.chars.peek().copied() {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::Arrow,
                            line: self.line,
                            column: self.column,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Subtract,
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                '*' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Multiply,
                        line: self.line,
                        column: self.column,
                    });
                }
                '<' => {
                    self.advance();
                    if let Some((_, '=')) = self.chars.peek().copied() {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::LessThanOrEqual,
                            line: self.line,
                            column: self.column,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::LessThan,
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                '>' => {
                    self.advance();
                    if let Some((_, '=')) = self.chars.peek().copied() {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::GreaterThanOrEqual,
                            line: self.line,
                            column: self.column,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::GreaterThan,
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                '!' => {
                    self.advance();
                    if let Some((_, '=')) = self.chars.peek().copied() {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::NotEqual,
                            line: self.line,
                            column: self.column,
                        });
                    } else {
                        return Err(LexError::UnexpectedChar('!', self.line, self.column));
                    }
                }
                ':' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Colon,
                        line: self.line,
                        column: self.column,
                    });
                }

                // String literals
                '"' => {
                    let start_col = self.column;
                    self.advance(); // consume quote
                    let mut value = String::new();

                    while let Some((_, ch)) = self.chars.peek().copied() {
                        if ch == '"' {
                            self.advance();
                            break;
                        }
                        value.push(ch);
                        self.advance();
                    }

                    if !value.ends_with('"') && !self.input[idx..].contains('"') {
                        return Err(LexError::UnterminatedString(self.line, start_col));
                    }

                    tokens.push(Token {
                        kind: TokenKind::StringLiteral(value),
                        line: self.line,
                        column: start_col,
                    });
                }

                // Numbers
                '0'..='9' => {
                    let start_col = self.column;
                    let mut value = String::new();
                    let mut dot_count = 0;

                    while let Some((_, ch)) = self.chars.peek().copied() {
                        if ch == '.' {
                            dot_count += 1;
                        }
                        if !ch.is_digit(10) && ch != '.' {
                            break;
                        }
                        value.push(ch);
                        self.advance();
                    }

                    if dot_count > 1 {
                        return Err(LexError::InvalidNumber(value, self.line, start_col));
                    }

                    if dot_count == 1 {
                        tokens.push(Token {
                            kind: TokenKind::FloatLiteral(value.parse().unwrap()),
                            line: self.line,
                            column: start_col,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::IntegerLiteral(value.parse().unwrap()),
                            line: self.line,
                            column: start_col,
                        });
                    }
                }

                // Identifiers / keywords / booleans
                ch if ch.is_alphabetic() || ch == '_' => {
                    let start_col = self.column;
                    let mut ident = String::new();
                    while let Some((_, ch)) = self.chars.peek().copied() {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let kind = match ident.as_str() {
                        "let" => TokenKind::Let,
                        "const" => TokenKind::Const,
                        "if" => TokenKind::If,
                        "elif" => TokenKind::ElseIf,
                        "else" => TokenKind::Else,
                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,
                        "return" => TokenKind::Return,
                        "fn" => TokenKind::Function,
                        "spawn" => TokenKind::Spawn,
                        "wait" => TokenKind::Wait,
                        "True" => TokenKind::BooleanLiteral(true),
                        "False" => TokenKind::BooleanLiteral(false),
                        "Int" => TokenKind::Integer,
                        "Float" => TokenKind::Float,
                        "String" => TokenKind::String,
                        "Bool" => TokenKind::Boolean,
                        "Array" => TokenKind::Array,
                        "Tuple" => TokenKind::Tuple,
                        "HashMap" => TokenKind::HashMap,
                        _ => TokenKind::Identifier(ident),
                    };

                    tokens.push(Token {
                        kind,
                        line: self.line,
                        column: start_col,
                    });
                }

                // Error
                ch => {
                    return Err(LexError::UnexpectedChar(ch, self.line, self.column));
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }
}
