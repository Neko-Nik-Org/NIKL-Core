use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Expr>),
    HashMap(Vec<(Expr, Expr)>),
    Tuple(Vec<Expr>),
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
    DotAccess {
        object: Box<Expr>,
        property: String,
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
        else_if_branches: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    Return(Expr),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Loop(Vec<Stmt>),
    Import {
        path: String,
        alias: String,
    },
    Delete(String),
    Break,
    Continue,
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
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Function => self.parse_function(),
            TokenKind::Import => self.parse_import(),
            TokenKind::Delete => self.parse_delete(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::Return => self.parse_return(),
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_break(&mut self) -> Result<Stmt, String> {
        self.advance();
        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> Result<Stmt, String> {
        self.advance();
        Ok(Stmt::Continue)
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance();
        let expr = self.parse_expr()?;
        Ok(Stmt::Return(expr))
    }

    fn parse_var_decl(&mut self, is_mut: bool) -> Result<Stmt, String> {
        self.advance();
        let name = if let TokenKind::Identifier(name) = &self.current().kind {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected identifier".to_string());
        };

        if matches!(self.current().kind, TokenKind::Colon) {
            self.advance();
            self.consume_type_annotation()?;
        }

        self.expect(&TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        if is_mut {
            Ok(Stmt::Let { name, value: expr })
        } else {
            Ok(Stmt::Const { name, value: expr })
        }
    }

    fn parse_delete(&mut self) -> Result<Stmt, String> {
        self.advance();
        let name = if let TokenKind::Identifier(name) = &self.current().kind {
            let n = name.clone();
            self.advance();
            n
        } else {
            return Err("Expected identifier".to_string());
        };
        Ok(Stmt::Delete(name))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // Consume 'if'
        let condition = self.parse_expr()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut body = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RightBrace)?;

        // Collect all else if branches
        let mut else_if_branches = Vec::new();
        while matches!(self.current().kind, TokenKind::ElseIf) {
            self.advance(); // Consume 'else if'
            let elif_cond = self.parse_expr()?;
            self.expect(&TokenKind::LeftBrace)?;

            let mut elif_body = Vec::new();
            while self.current().kind != TokenKind::RightBrace {
                elif_body.push(self.parse_stmt()?);
            }
            self.expect(&TokenKind::RightBrace)?;
            else_if_branches.push((elif_cond, elif_body));
        }

        // Optional else
        let else_body = if matches!(self.current().kind, TokenKind::Else) {
            self.advance();
            self.expect(&TokenKind::LeftBrace)?;
            let mut stmts = Vec::new();
            while self.current().kind != TokenKind::RightBrace {
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
            else_if_branches,
            else_body,
        })
    }

    fn parse_loop(&mut self) -> Result<Stmt, String> {
        // Example: loop { ... }
        self.advance(); // Consume 'loop'
        self.expect(&TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(Stmt::Loop(body))
    }

    fn consume_type_annotation(&mut self) -> Result<(), String> {
        // TODO: Store the type annotation in the AST
        use TokenKind::*;
        match &self.current().kind {
            Integer | Float | String | Boolean | Array | HashMap | Tuple | Identifier(_) => {
                self.advance();
            }
            LeftBracket => {
                self.advance();
                self.expect(&RightBracket)?;
            }
            LeftParen => {
                self.advance();
                self.expect(&RightParen)?;
            }
            other => {
                return Err(format!("Expected type annotation, but found {:?}", other));
            }
        }
        Ok(())
    }

    fn parse_function_signature(&mut self) -> Result<(String, Vec<String>), String> {
        self.advance();
        let name = match &self.current().kind {
            TokenKind::Identifier(name) => {
                let n = name.clone();
                self.advance();
                n
            }
            _ => return Err("Expected function name".to_string()),
        };

        self.expect(&TokenKind::LeftParen)?;
        let mut params = Vec::new();

        while !matches!(self.current().kind, TokenKind::RightParen) {
            let param = match &self.current().kind {
                TokenKind::Identifier(name) => {
                    let p = name.clone();
                    self.advance();
                    p
                }
                _ => return Err("Expected parameter name".to_string()),
            };

            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance();
                self.consume_type_annotation()?;
            }

            params.push(param);

            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(&TokenKind::RightParen)?;

        if matches!(self.current().kind, TokenKind::Arrow) {
            self.advance();
            self.consume_type_annotation()?;
        }

        Ok((name, params))
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        let (name, params) = self.parse_function_signature()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut body = Vec::new();
        while self.current().kind != TokenKind::RightBrace {
            body.push(self.parse_stmt()?);
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Stmt::Function { name, params, body })
    }

    fn parse_import(&mut self) -> Result<Stmt, String> {
        self.advance();
        let path = if let TokenKind::StringLiteral(path) = &self.current().kind {
            let p = path.clone();
            self.advance();
            p
        } else {
            return Err("Expected string literal for import path".to_string());
        };

        self.expect(&TokenKind::As)?;
        let alias = if let TokenKind::Identifier(alias) = &self.current().kind {
            let a = alias.clone();
            self.advance();
            a
        } else {
            return Err("Expected identifier for import alias".to_string());
        };

        Ok(Stmt::Import { path, alias })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;

        if matches!(self.current().kind, TokenKind::Assign) {
            if let Expr::Identifier(name) = expr {
                self.advance();
                let value = self.parse_assignment()?;
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
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current().kind.clone() {
                TokenKind::Dot => {
                    self.advance();
                    if let TokenKind::Identifier(name) = &self.current().kind {
                        let prop = name.clone();
                        self.advance();
                        expr = Expr::DotAccess {
                            object: Box::new(expr),
                            property: prop,
                        };
                    } else {
                        return Err("Expected identifier after '.'".to_string());
                    }
                }
                TokenKind::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.current().kind, TokenKind::RightParen) {
                        args.push(self.parse_expr()?);
                        if matches!(self.current().kind, TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RightParen)?;
                    expr = Expr::Call {
                        function: Box::new(expr),
                        args,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
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
                Ok(Expr::Identifier(name.clone()))
            }
            TokenKind::LeftParen => {
                self.advance();
                let mut elements = Vec::new();
                if !matches!(self.current().kind, TokenKind::RightParen) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if matches!(self.current().kind, TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RightParen)?;
                if elements.len() == 1 {
                    Ok(elements.remove(0))
                } else {
                    Ok(Expr::Tuple(elements))
                }
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !matches!(self.current().kind, TokenKind::RightBracket) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if matches!(self.current().kind, TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut pairs = Vec::new();
                if !matches!(self.current().kind, TokenKind::RightBrace) {
                    loop {
                        let key = self.parse_expr()?;
                        self.expect(&TokenKind::Colon)?;
                        let value = self.parse_expr()?;
                        pairs.push((key, value));
                        if matches!(self.current().kind, TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RightBrace)?;
                Ok(Expr::HashMap(pairs))
            }
            _ => Err(format!(
                "Unexpected token: {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    }
}
