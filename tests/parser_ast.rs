use nikl::lexer::Lexer;
use nikl::parser::{Parser, Stmt, Expr};


fn parse_input(source: &str) -> Result<Vec<Stmt>, String> {
    let lexer = Lexer::new(source);
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
        Stmt::If { condition, body, else_if_branches, else_body } => {
            assert!(matches!(condition, Expr::Bool(true)));
            assert!(body.len() == 1);
            assert!(else_if_branches.is_empty());
            assert!(else_body.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_if_statement_with_elif() {
    let source = r#"
        if x > 10 {
            print("x is large")
        } elif x < 5 {
            print("x is small")
        }
    "#;
    let ast = parse_input(source).unwrap();
    match &ast[0] {
        Stmt::If { condition, body, else_if_branches, else_body } => {
            assert!(matches!(condition, Expr::BinaryOp { .. }));
            assert!(body.len() == 1);
            assert!(else_if_branches.len() == 1);
            assert!(else_body.is_none());
        }
        _ => panic!("Expected if/else if statement"),
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
        Stmt::If { condition, body, else_if_branches, else_body } => {
            assert!(matches!(condition, Expr::Bool(false)));
            assert_eq!(body.len(), 1);
            assert!(else_if_branches.is_empty());
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
            print("small")  // Test comment
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

#[test]
fn test_single_print_statement() {
    let source = "print(42)";
    let ast = parse_input(source).unwrap();
    assert_eq!(ast.len(), 1);
    assert!(matches!(ast[0], Stmt::Expr(Expr::Call { .. })));
}
