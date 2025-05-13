// src/parser.rs
use crate::lexer::Token;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Sequence(Vec<Expr>),
    Print(Box<Expr>),
    Let(String, Box<Expr>),
    Spawn(Box<Expr>),
    Wait(Box<Expr>),
    Identifier(String),
    StringLiteral(String),
    Call(String, Vec<Expr>),
}



pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut iter = tokens.into_iter().peekable();
    let mut expressions = vec![];

    while let Some(token) = iter.peek() {
        let expr = match token {
            Token::Let => {
                iter.next(); // consume Let
                if let Some(Token::Identifier(name)) = iter.next() {
                    if let Some(Token::Equals) = iter.next() {
                        if let Some(Token::StringLiteral(value)) = iter.next() {
                            Expr::Let(name, Box::new(Expr::StringLiteral(value)))
                        } else {
                            panic!("Expected value after equals")
                        }
                    } else {
                        panic!("Expected equals after identifier")
                    }
                } else {
                    panic!("Expected identifier after 'let'")
                }
            }
            Token::Print | Token::Spawn | Token::Wait => {
                let keyword = iter.next().unwrap();
                if let Some(Token::LeftParen) = iter.next() {
                    let arg = match iter.next() {
                        Some(Token::Identifier(name)) => Expr::Identifier(name),
                        Some(Token::StringLiteral(value)) => Expr::StringLiteral(value),
                        _ => panic!("Expected argument inside parentheses"),
                    };
                    if let Some(Token::RightParen) = iter.next() {
                        match keyword {
                            Token::Print => Expr::Print(Box::new(arg)),
                            Token::Spawn => Expr::Spawn(Box::new(arg)),
                            Token::Wait => Expr::Wait(Box::new(arg)),
                            _ => unreachable!(),
                        }
                    } else {
                        panic!("Expected closing parenthesis")
                    }
                } else {
                    panic!("Expected opening parenthesis")
                }
            }
            Token::Eof => {
                iter.next(); // consume Eof
                break;
            }
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        };

        expressions.push(expr);
    }

    if expressions.len() == 1 {
        expressions.remove(0)
    } else {
        Expr::Sequence(expressions)
    }
}
