use crate::lexer::{Token, TokenKind};


#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr>,
    },
    UnaryOp {
        op: TokenKind,
        expr: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Const { name: String, value: Expr },
    Expr(Expr),
    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    Return(Expr),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(self.tokens.last().unwrap())
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: &TokenKind) -> Result<(), String> {
        if &self.current().kind == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at line {}, column {}",
                expected, self.current().kind, self.current().line, self.current().column
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while self.current().kind != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match &self.current().kind {
            TokenKind::Let => self.parse_var_decl(true),
            TokenKind::Const => self.parse_var_decl(false),
            TokenKind::If => self.parse_if(),
            TokenKind::Function => self.parse_function(),
            TokenKind::LeftBrace => {
                self.advance(); // consume '{'
                let mut body = Vec::new();
                while self.current().kind != TokenKind::RightBrace && self.current().kind != TokenKind::Eof {
                    body.push(self.parse_stmt()?);
                }
                self.expect(&TokenKind::RightBrace)?;
                Ok(Stmt::Expr(Expr::Call {
                    function: Box::new(Expr::Identifier("block".to_string())),
                    args: body.into_iter().map(|s| Expr::Identifier(format!("{:?}", s))).collect(),
                }))
            }
            TokenKind::RightBrace => {
                self.advance(); // consume '}'
                Ok(Stmt::Expr(Expr::Identifier("}".to_string())))
            }
            TokenKind::Return => {
                self.advance(); // consume 'return'
                let expr = self.parse_expr()?;
                Ok(Stmt::Return(expr))
            }
            TokenKind::Eof => Ok(Stmt::Expr(Expr::Bool(true))),
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_var_decl(&mut self, is_mut: bool) -> Result<Stmt, String> {
        self.advance(); // consume let or const
        let name = if let TokenKind::Identifier(name) = &self.current().kind {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected identifier".to_string());
        };
        self.expect(&TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        if is_mut {
            Ok(Stmt::Let { name, value: expr })
        } else {
            Ok(Stmt::Const { name, value: expr })
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'if'
        let condition = self.parse_expr()?;
        self.expect(&TokenKind::LeftBrace)?; // start block

        let mut body = Vec::new();
        while self.current().kind != TokenKind::RightBrace && self.current().kind != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }

        self.expect(&TokenKind::RightBrace)?; // end block

        let else_body = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            self.expect(&TokenKind::LeftBrace)?;
            let mut stmts = Vec::new();
            while self.current().kind != TokenKind::RightBrace && self.current().kind != TokenKind::Eof {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(&TokenKind::RightBrace)?;
            Some(stmts)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            body,
            else_body,
        })
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'function'
        let name = if let TokenKind::Identifier(name) = &self.current().kind {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected function name after 'function'".to_string());
        };
    
        self.expect(&TokenKind::LeftParen)?;
        let mut params = Vec::new();
        while !matches!(self.current().kind, TokenKind::RightParen) {
            if let TokenKind::Identifier(param) = &self.current().kind {
                params.push(param.clone());
                self.advance();
                if matches!(self.current().kind, TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                return Err("Expected parameter name".to_string());
            }
        }
        self.expect(&TokenKind::RightParen)?;
        self.expect(&TokenKind::LeftBrace)?;
    
        let mut body = Vec::new();
        while !matches!(self.current().kind, TokenKind::RightBrace) {
            if self.current().kind == TokenKind::Eof {
                return Err("Unexpected end of file".to_string());
            }
            body.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RightBrace)?;
    
        Ok(Stmt::Function {
            name,
            params,
            body,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?; // Left-hand side

        if matches!(self.current().kind, TokenKind::Assign) {
            if let Expr::Identifier(name) = expr {
                self.advance(); // consume '='
                let value = self.parse_assignment()?; // right-hand side
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                return Err("Invalid assignment target".to_string());
            }
        }
    
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and()?;
        while matches!(self.current().kind, TokenKind::Or) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_equality()?;
        while matches!(self.current().kind, TokenKind::And) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;
        while matches!(
            self.current().kind,
            TokenKind::Equals | TokenKind::NotEqual
        ) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        while matches!(
            self.current().kind,
            TokenKind::LessThan
                | TokenKind::GreaterThan
                | TokenKind::LessThanOrEqual
                | TokenKind::GreaterThanOrEqual
        ) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        while matches!(self.current().kind, TokenKind::Add | TokenKind::Subtract) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        while matches!(self.current().kind, TokenKind::Multiply | TokenKind::Divide) {
            let op = self.current().kind.clone();
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.current().kind, TokenKind::Subtract | TokenKind::Not) {
            let op = self.current().kind.clone();
            self.advance();
            let expr = self.parse_unary()?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.current().clone();
        match token.kind {
            TokenKind::IntegerLiteral(val) => {
                self.advance();
                Ok(Expr::Integer(val))
            }
            TokenKind::FloatLiteral(val) => {
                self.advance();
                Ok(Expr::Float(val))
            }
            TokenKind::BooleanLiteral(val) => {
                self.advance();
                Ok(Expr::Bool(val))
            }
            TokenKind::StringLiteral(ref s) => {
                self.advance();
                Ok(Expr::String(s.clone()))
            }
            TokenKind::Identifier(ref name) => {
                self.advance();
                if matches!(self.current().kind, TokenKind::LeftParen) {
                    self.advance(); // (
                    let mut args = Vec::new();
                    while !matches!(self.current().kind, TokenKind::RightParen) {
                        args.push(self.parse_expr()?);
                        if matches!(self.current().kind, TokenKind::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(&TokenKind::RightParen)?;
                    Ok(Expr::Call {
                        function: Box::new(Expr::Identifier(name.clone())),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name.clone()))
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token: {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    }
}
