#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Diclaration keywords
    Let,
    Const,
    Function,
    Import,
    Pub,
    As,
    In,
    For,
    While,
    Loop,
    Break,
    Continue,
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
    Delete,

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
    Dot,

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

    fn add_token(&mut self, tokens: &mut Vec<Token>, kind: TokenKind, col: usize) {
        tokens.push(Token {
            kind,
            line: self.line,
            column: col,
        });
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(&(idx, ch)) = self.chars.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }

                // Comments: //
                '/' => {
                    self.advance();
                    if let Some(&(_, '/')) = self.chars.peek() {
                        // Consume till newline
                        while let Some(&(_, c)) = self.chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        self.add_token(&mut tokens, TokenKind::Divide, self.column);
                    }
                }

                // Multi-char operators and single-char symbols
                '=' => {
                    let col = self.column;
                    self.advance();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.advance();
                        self.add_token(&mut tokens, TokenKind::Equals, col);
                    } else {
                        self.add_token(&mut tokens, TokenKind::Assign, col);
                    }
                }

                '!' => {
                    let col = self.column;
                    self.advance();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.advance();
                        self.add_token(&mut tokens, TokenKind::NotEqual, col);
                    } else {
                        return Err(LexError::UnexpectedChar('!', self.line, col));
                    }
                }

                '<' => {
                    let col = self.column;
                    self.advance();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.advance();
                        self.add_token(&mut tokens, TokenKind::LessThanOrEqual, col);
                    } else {
                        self.add_token(&mut tokens, TokenKind::LessThan, col);
                    }
                }

                '>' => {
                    let col = self.column;
                    self.advance();
                    if let Some(&(_, '=')) = self.chars.peek() {
                        self.advance();
                        self.add_token(&mut tokens, TokenKind::GreaterThanOrEqual, col);
                    } else {
                        self.add_token(&mut tokens, TokenKind::GreaterThan, col);
                    }
                }

                '-' => {
                    let col = self.column;
                    self.advance();
                    if let Some(&(_, '>')) = self.chars.peek() {
                        self.advance();
                        self.add_token(&mut tokens, TokenKind::Arrow, col);
                    } else {
                        self.add_token(&mut tokens, TokenKind::Subtract, col);
                    }
                }

                // Single char tokens
                '(' => { self.advance(); self.add_token(&mut tokens, TokenKind::LeftParen, self.column -1); }
                ')' => { self.advance(); self.add_token(&mut tokens, TokenKind::RightParen, self.column -1); }
                '{' => { self.advance(); self.add_token(&mut tokens, TokenKind::LeftBrace, self.column -1); }
                '}' => { self.advance(); self.add_token(&mut tokens, TokenKind::RightBrace, self.column -1); }
                '[' => { self.advance(); self.add_token(&mut tokens, TokenKind::LeftBracket, self.column -1); }
                ']' => { self.advance(); self.add_token(&mut tokens, TokenKind::RightBracket, self.column -1); }
                ',' => { self.advance(); self.add_token(&mut tokens, TokenKind::Comma, self.column -1); }
                '+' => { self.advance(); self.add_token(&mut tokens, TokenKind::Add, self.column -1); }
                '*' => { self.advance(); self.add_token(&mut tokens, TokenKind::Multiply, self.column -1); }
                ':' => { self.advance(); self.add_token(&mut tokens, TokenKind::Colon, self.column -1); }
                '.' => { self.advance(); self.add_token(&mut tokens, TokenKind::Dot, self.column -1); }

                // String literals
                '"' => {
                    let start_col = self.column;
                    self.advance(); // consume opening quote
                    let mut value = String::new();

                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch == '"' {
                            self.advance(); // consume closing quote
                            break;
                        }
                        value.push(ch);
                        self.advance();
                    }

                    // Check if closed properly
                    if !self.input[idx..].contains('"') && !self.input[idx..].ends_with('"') {
                        return Err(LexError::UnterminatedString(self.line, start_col));
                    }

                    self.add_token(&mut tokens, TokenKind::StringLiteral(value), start_col);
                }

                // Numbers (int or float)
                '0'..='9' => {
                    let start_col = self.column;
                    let mut num_str = String::new();
                    let mut dot_count = 0;

                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch == '.' {
                            dot_count += 1;
                            if dot_count > 1 {
                                break;
                            }
                        } else if !ch.is_ascii_digit() {
                            break;
                        }
                        num_str.push(ch);
                        self.advance();
                    }

                    if dot_count == 1 {
                        match num_str.parse::<f64>() {
                            Ok(f) => self.add_token(&mut tokens, TokenKind::FloatLiteral(f), start_col),
                            Err(_) => return Err(LexError::InvalidNumber(num_str, self.line, start_col)),
                        }
                    } else {
                        match num_str.parse::<i64>() {
                            Ok(i) => self.add_token(&mut tokens, TokenKind::IntegerLiteral(i), start_col),
                            Err(_) => return Err(LexError::InvalidNumber(num_str, self.line, start_col)),
                        }
                    }
                }

                // Identifiers, keywords, booleans
                ch if ch.is_alphabetic() || ch == '_' => {
                    let start_col = self.column;
                    let mut ident = String::new();

                    while let Some(&(_, ch)) = self.chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let kind = match ident.as_str() {
                        "import" => TokenKind::Import,
                        "pub" => TokenKind::Pub,
                        "as" => TokenKind::As,

                        "let" => TokenKind::Let,
                        "const" => TokenKind::Const,
                        "fn" => TokenKind::Function,
                        "spawn" => TokenKind::Spawn,
                        "wait" => TokenKind::Wait,
                        "return" => TokenKind::Return,
                        "del" => TokenKind::Delete,
                        "in" => TokenKind::In,

                        "if" => TokenKind::If,
                        "elif" => TokenKind::ElseIf,
                        "else" => TokenKind::Else,
                        "for" => TokenKind::For,
                        "while" => TokenKind::While,
                        "loop" => TokenKind::Loop,
                        "break" => TokenKind::Break,
                        "continue" => TokenKind::Continue,

                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,

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

                    self.add_token(&mut tokens, kind, start_col);
                }

                _ => {
                    return Err(LexError::UnexpectedChar(ch, self.line, self.column));
                }
            }
        }

        self.add_token(&mut tokens, TokenKind::Eof, self.column);
        Ok(tokens)
    }
}
