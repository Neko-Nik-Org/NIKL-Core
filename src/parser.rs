use crate::lexer::{Token, TokenKind};


#[derive(Debug)]
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

#[derive(Debug)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Const { name: String, value: Expr },
    Print(Expr),
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
            TokenKind::Comment(_) => {
                self.advance();
                self.parse_stmt()
            }
            TokenKind::Let => self.parse_var_decl(true),
            TokenKind::Const => self.parse_var_decl(false),
            TokenKind::Print => self.parse_print(),
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

    fn parse_print(&mut self) -> Result<Stmt, String> {
        // Print should be always be: print(<expr>)
        self.advance(); // consume 'print'
        self.expect(&TokenKind::LeftParen)?;
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::RightParen)?;
        Ok(Stmt::Print(expr))
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




// --- Test cases ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Lexer, TokenKind};

    fn parse_input(source: &str) -> Result<Vec<Stmt>, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap(); // or handle errors
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_let_statement() {
        let source = "let x = 5 + 3";
        let ast = parse_input(source).unwrap();
        assert!(matches!(ast[0], Stmt::Let { .. }));
    }

    #[test]
    fn test_function_declaration() {
        let source = r#"
            fn add(a, b) {
                return a + b
            }
        "#;
        let ast = parse_input(source).unwrap();
        assert!(matches!(ast[0], Stmt::Expr(Expr::Call { .. }) | Stmt::Function { .. }));
    }

    #[test]
    fn test_const_statement() {
        let source = "const y = True";
        let ast = parse_input(source).unwrap();
        assert!(matches!(ast[0], Stmt::Const { .. }));
    }

    #[test]
    fn test_assignment_expression() {
        let source = "x = 42";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Expr(Expr::Assign { name, value }) => {
                assert_eq!(name, "x");
                assert!(matches!(**value, Expr::Integer(42)));
            }
            _ => panic!("Expected assignment expression"),
        }
    }

    #[test]
    fn test_binary_expression_precedence() {
        let source = "1 + 2 * 3";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Expr(Expr::BinaryOp { .. }) => {} // good enough here
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_unary_expression() {
        let source = "not False";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Expr(Expr::UnaryOp { .. }) => {}
            _ => panic!("Expected unary operation"),
        }
    }

    #[test]
    fn test_grouping_expression() {
        let source = "(1 + 2) * 3";
        let ast = parse_input(source).unwrap();
        assert!(matches!(ast[0], Stmt::Expr(_)));
    }

    #[test]
    fn test_function_call() {
        let source = "foo(1, 2, 3)";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Expr(Expr::Call { function, args }) => {
                assert!(matches!(**function, Expr::Identifier(ref name) if name == "foo"));
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_if_statement_without_else() {
        let source = r#"
            if True {
                print(1)
            }
        "#;
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::If { condition, body, else_body } => {
                assert!(matches!(condition, Expr::Bool(true)));
                assert!(body.len() == 1);
                assert!(else_body.is_none());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_if_statement_with_else() {
        let source = r#"
            if False {
                print(1)
            } else {
                print(2)
            }
        "#;
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::If { condition, body, else_body } => {
                assert!(matches!(condition, Expr::Bool(false)));
                assert_eq!(body.len(), 1);
                assert!(else_body.as_ref().unwrap().len() == 1);
            }
            _ => panic!("Expected if/else statement"),
        }
    }

    #[test]
    fn test_return_statement() {
        let source = "return 123";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Return(expr) => {
                assert!(matches!(expr, Expr::Integer(123)));
            }
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn test_print_statement() {
        let source = "print(\"hello\")";
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Print(expr) => {
                assert!(matches!(expr, Expr::String(s) if s == "hello"));
            }
            _ => panic!("Expected print statement"),
        }
    }

    #[test]
    fn test_block_statement() {
        let source = r#"
            {
                print(1)
                print(2)
            }
        "#;
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Expr(Expr::Call { function, args }) => {
                if let Expr::Identifier(name) = &**function {
                    assert_eq!(name, "block");
                    assert!(args.len() == 2);
                } else {
                    panic!("Expected block identifier");
                }
            }
            _ => panic!("Expected block statement as function call"),
        }
    }

    #[test]
    fn test_function_declaration_with_params() {
        let source = r#"
            fn greet(name, age) {
                print(name)
                print(age)
            }
        "#;
        let ast = parse_input(source).unwrap();
        match &ast[0] {
            Stmt::Function { name, params, body } => {
                assert_eq!(name, "greet");
                assert_eq!(params, &vec!["name".to_string(), "age".to_string()]);
                assert_eq!(body.len(), 2);
            }
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn test_nested_expressions() {
        let source = "1 + (2 * (3 + 4))";
        let ast = parse_input(source).unwrap();
        assert!(matches!(ast[0], Stmt::Expr(_))); // no panic = success
    }

    #[test]
    fn test_complex_program() {
        let source = r#"
            // 1
            const x = 10
            // 2
            let y = x + 5
            // 3
            if y > 10 {
                print(y)
            } else {
                print("small")
            }
            // 4
            fn add(a, b) {
                return a + b
            }
            // 5
            let result = add(2, 3)
            // 6
            print(result)
        "#;
        let ast = parse_input(source).unwrap();
        assert_eq!(ast.len(), 6);
    }

}
