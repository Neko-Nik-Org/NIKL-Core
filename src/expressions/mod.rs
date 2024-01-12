
// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;


use crate::lexer::make_tokens::{Token, TokenType};


#[derive(Debug)]
pub enum BinaryOperator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    StarStar
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum Expression {
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expression> },
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Grouping { expression: Box<Expression> },
    Print { expression: Box<Expression> }
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Assignment { variable: Token, value: Expression , is_constant: bool},
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            let statement = self.statement()?;
            statements.push(statement);
        }
        
        Ok(statements)
    }

    // Parse the highest precedence expression
    fn expression(&mut self) -> Result<Expression, String> {
        self.equality()
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Expression(expr))
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(vec![TokenType::Var]) {
            self.var_declaration()
        }

        else if self.match_token(vec![TokenType::Const]) {
            self.const_declaration()
        }

        else if self.match_token(vec![TokenType::Semicolon]) {
            Ok(Statement::Expression(Expression::Literal { value: Literal::Nil }))
        }
   
        else if self.match_token(vec![TokenType::Print]) {
            self.print_statement()
        }
        
        // Add all the other statements here that you want to parse
        else if false {
            todo!("Add all the other statements here that you want to parse")
        }

        else {
            self.expression_statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, String> {
        let variable = self.consume(TokenType::Identifier, "Expect variable name after 'var'.")?;
        let initializer = if self.match_token(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Statement::Assignment {
            variable,
            value: initializer.unwrap_or_else(|| Expression::Literal { value: Literal::Nil }),
            is_constant: false,
        })
    }

    fn const_declaration(&mut self) -> Result<Statement, String> {
        let constant = self.consume(TokenType::Identifier, "Expect constant name after 'const'.")?;
        let initializer = if self.match_token(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            // Constant must be initialized
            return Err("Expect constant to be initialized.".to_string());
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Statement::Assignment {
            variable: constant,
            value: initializer.unwrap_or_else(|| Expression::Literal { value: Literal::Nil }),
            is_constant: true,
        })
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut left_expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = match self.previous().token_type {
                TokenType::BangEqual => BinaryOperator::BangEqual,
                TokenType::EqualEqual => BinaryOperator::EqualEqual,
                _ => unreachable!(),
            };

            let right_expr = self.comparison()?;
            left_expr = Expression::Binary {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr),
            };
        }

        Ok(left_expr)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut left_expr = self.addition()?;

        while self.match_token(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = match self.previous().token_type {
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                _ => unreachable!(),
            };

            let right_expr = self.addition()?;
            left_expr = Expression::Binary {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr),
            };
        }

        Ok(left_expr)
    }

    fn addition(&mut self) -> Result<Expression, String> {
        let mut left_expr = self.multiplication()?;

        while self.match_token(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOperator::Plus,
                TokenType::Minus => BinaryOperator::Minus,
                _ => unreachable!(),
            };

            let right_expr = self.multiplication()?;
            left_expr = Expression::Binary {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr),
            };
        }

        Ok(left_expr)
    }

    fn multiplication(&mut self) -> Result<Expression, String> {
        let mut left_expr = self.unary()?;

        while self.match_token(vec![TokenType::Star, TokenType::Slash, TokenType::StarStar]) {
            let operator = match self.previous().token_type {
                TokenType::StarStar => BinaryOperator::StarStar,
                TokenType::Star => BinaryOperator::Star,
                TokenType::Slash => BinaryOperator::Slash,
                _ => unreachable!(),
            };

            let right_expr = self.unary()?;
            left_expr = Expression::Binary {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr),
            };
        }

        Ok(left_expr)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right_expr = self.unary()?;
            Ok(Expression::Unary {
                operator,
                right: Box::new(right_expr),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_token(vec![TokenType::Number, TokenType::String, TokenType::True, TokenType::False, TokenType::Nil]) {
            let literal = match self.previous().token_type {
                TokenType::Number => Literal::Number(self.previous().lexeme.parse().unwrap()),
                TokenType::String => Literal::String(self.previous().lexeme.clone()),
                TokenType::True => Literal::Boolean(true),
                TokenType::False => Literal::Boolean(false),
                TokenType::Nil => Literal::Nil,
                _ => unreachable!(),
            };
            Ok(Expression::Literal { value: literal })
        } else if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expression::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(format!("Unexpected token: {:?}", self.peek()))
        }
    }

    fn print_statement(&mut self) -> Result<Statement, String> {
        // print(expr or variable name or literal or Identifier)
        self.consume(TokenType::LeftParen, format!("Expected '(' after 'print' but found {:?}", self.peek().lexeme).as_str())?;
        let expr = self.expression()?;

        self.consume(TokenType::RightParen, format!("Expected ')' after 'print' but found {:?}", self.peek().lexeme).as_str())?;
        self.consume(TokenType::Semicolon, format!("Expected ';' after 'print' but found {:?}", self.peek().lexeme).as_str())?;

        Ok(Statement::Expression(Expression::Print { expression: Box::new(expr) }))
    }

    // Helper functions

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(error_message.to_string())
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
