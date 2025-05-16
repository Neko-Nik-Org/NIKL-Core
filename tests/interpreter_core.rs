use nikl::run_script;


#[test]
fn test_variable_declaration_and_assignment() {
    let input = r#"
        let x = 10
        let y = 20
        x = x + y
        print(x)    // should print 30
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_constants() {
    let input = r#"
        const x = 42
        print(x)
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_assignment_error_on_const() {
    let input = r#"
        const x = 5;
        x = 10;
    "#;

    let result = run_script(input);
    assert!(result.is_err());
}

#[test]
fn test_binary_operations() {
    let input = r#"
        let a = 5 + 2 * 3
        let b = a - 4 / 2
        print(b)    // should print 9
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_statement_true_branch() {
    let input = r#"
        let x = 10
        if (x > 5) {
            print("greater")
        } else {
            print("less")
        }
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_statement_false_branch() {
    let input = r#"
        let x = 3
        if (x > 5) {
            print("greater")
        } else {
            print("less")
        }
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_function_definition_and_call() {
    let input = r#"
        fn add(a, b) {
            return a + b
        }

        let result = add(3, 4)
        print(result)   // should print 7
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_function_with_if_and_return() {
    let input = r#"
        fn max(a, b) {
            if (a > b) {
                return a
            } else {
                return b
            }
        }

        let m = max(7, 4)
        print(m)    // should print 7
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_nested_function_calls() {
    let input = r#"
        fn square(x) {
            return x * x
        }

        fn double(x) {
            return x + x
        }

        let result = square(double(3))
        print(result)   // should print 36
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_print_function() {
    let input = r#"
        print("Hello, World!")
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_length_function() {
    let input = r#"
        let str = "Hello"
        let len_ = len(str)
        print(len_)    // should print 5
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_variable_shadowing_in_nested_scope() {
    let input = r#"
        let x = 5
        fn foo() {
            let x = 10
            print(x)    // should print 10
        }
        foo()
        print(x)        // should print 5
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_division_by_zero() {
    let input = r#"
        let a = 10 / 0
        print(a)
    "#;

    let result = run_script(input);
    assert!(result.is_err());
}

#[test]
fn test_boolean_logic_operations() {
    let input = r#"
        let a = True and False
        let b = not a
        let c = b or False
        print(c)    // should print True
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_function_closure_scope() {
    let input = r#"
        let x = 100

        fn show(y) {
            print(x + y)    // should print 100 because of closure
        }

        show(50)
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_fn_type_hinting_1() {
    let input = r#"
        fn add(a: int, b: int) -> int {
            return a + b
        }

        let result = add(5, 10)
        print(result)   // should print 15
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_fn_type_hinting_2() {
    let input = r#"
        fn concat(a: str, b: bool, c: float, d: int) -> str {
            return a
        }
        concat("Hello", True, 3.14, 42)
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_fn_type_hinting_3() {
    let input = r#"
        fn add(a, b: int) -> int {
            return a + b
        }

        let result = add(5, 10)
        print(result)   // should print 15
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_fn_type_hinting_4() {
    let input = r#"
        fn add(a: int, b) -> int {
            return a + b
        }

        let result = add(5, 10)
        print(result)   // should print 15
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_fn_type_hinting_5() {
    let input = r#"
        let new_var_type = 5

        // Will allow anything as type hint (even if it is not a type)
        fn add(a, b: new_var_type) -> str {
            return a + b
        }

        let result = add(5, 10)
        print(result)   // should print 15
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}
