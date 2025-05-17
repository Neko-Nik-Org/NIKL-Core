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
        let string_ = "Hello"
        let len_ = len(string_)
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


#[test]
fn test_nested_function_definition() {
    let input = r#"
        fn outer() {
            fn inner() {
                print("Hello from inner function!")
            }
            inner()
        }
        outer()
    "#;

    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_print() {
    let input = r#"
        print("Hello, world!")
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_len() {
    let input = r#"
        let s = "hello"
        let l = len(s)
        print(l)  // Expect 5
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_str() {
    let input = r#"
        let n = 123
        let s = str(n)
        print(s)  // Expect "123"
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_int() {
    let input = r#"
        let f = 3.14
        let i = int(f)
        print(i)  // Expect 3
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_float() {
    let input = r#"
        let i = 7
        let f = float(i)
        print(f)  // Expect 7.0
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_bool() {
    let input = r#"
        let b1 = bool(0)
        let b2 = bool(1)
        print(b1)  // Expect false
        print(b2)  // Expect true
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_exit() {
    // Since exit terminates the process, you might want to mock or test the error case only.
    // let input = r#"
    //     exit(0)
    // "#;
    // This will terminate the test runner if actually called,
    // so you may want to skip or mock exit in tests.
    // Here we just check for argument errors:
    let bad_input = r#"
        exit("not an int")
    "#;
    let bad_result = run_script(bad_input);
    assert!(bad_result.is_err());
}

#[test]
fn test_type() {
    let input = r#"
        let x = 123
        print(type(x))  // Expect "int" or equivalent string
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_input() {
    // input() requires user interaction; testing it automatically is tricky.
    // You might want to mock input or test error cases:
    let bad_input = r#"
        input(123)
    "#;
    let bad_result = run_script(bad_input);
    assert!(bad_result.is_err());

    // For no-argument input(), you might skip or test manually.
}

#[test]
fn test_imports() {
    let input = r#"
        import "tests/sample.nk" as sample
        let result = sample.get_sample()
        print(result == sample.sample_exp)
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_imports_with_error() {
    let input = r#"
        import "tests/non_existent_file.nk" as sample
    "#;
    let result = run_script(input);
    assert!(result.is_err());
}

#[test]
fn test_imports_with_invalid_alias() {
    let input = r#"
        import "tests/sample.nk" as 123
    "#;
    let result = run_script(input);
    assert!(result.is_err());
}

#[test]
fn test_loop_break() {
    let input = r#"
        let sum = 0
        loop {
            sum = sum + 1
            if (sum >= 5) {
                break
            }
        }
        print(sum)  // Expect 5
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_loop_continue() {
    let input = r#"
        let sum = 0
        loop {
            sum = sum + 1
            if (sum == 3) {
                continue
            }
            if (sum >= 5) {
                break
            }
        }
        print(sum)  // Expect 5
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_while_loop() {
    let input = r#"
        let sum = 0
        let i = 0
        while (i < 5) {
            sum = sum + i
            i = i + 1
        }
        print(sum)  // Expect 10
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_while_loop_with_break() {
    let input = r#"
        let sum = 0
        let i = 0
        while (i < 10) {
            if (i == 5) {
                break
            }
            sum = sum + i
            i = i + 1
        }
        print(sum)  // Expect 10
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_while_loop_with_continue() {
    let input = r#"
        let sum = 0
        let i = 0
        while (i < 5) {
            i = i + 1
            if (i == 3) {
                continue
            }
            sum = sum + i
        }
        print(sum)  // Expect 12
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop() {
    let input = r#"
        let test_array = [1, 2, 3, 4, 5]

        for i in test_array {
            print(i)
        }
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_break() {
    let input = r#"
        let test_array = [1, 2, 3, 4, 5]
        let sum = 0

        for i in test_array {
            if (i == 3) {
                break
            }
            sum = sum + i
        }
        print(sum)  // Expect 3
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_continue() {
    let input = r#"
        let test_array = [1, 2, 3, 4, 5]
        let sum = 0

        for i in test_array {
            if (i == 3) {
                continue
            }
            sum = sum + i
        }
        print(sum)  // Expect 12
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_tuple() {
    let input = r#"
        let test_tuple = (1, 2, 3, 4, 5)
        let sum = 0
        for i in test_tuple {
            sum = sum + i
        }
        print(sum)  // Expect 15
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_tuple_and_break() {
    let input = r#"
        let test_tuple = (1, 2, 3, 4, 5)
        let sum = 0
        for i in test_tuple {
            if (i == 3) {
                break
            }
            sum = sum + i
        }
        print(sum)  // Expect 3
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_tuple_and_continue() {
    let input = r#"
        let test_tuple = (1, 2, 3, 4, 5)
        let sum = 0
        for i in test_tuple {
            if (i == 3) {
                continue
            }
            sum = sum + i
        }
        print(sum)  // Expect 12
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_string() {
    let input = r#"
        let test_string = "hello"
        let sum = 0
        for i in test_string {
            print(i)
            sum = sum + 1
        }
        print(sum)  // Expect the sum of ASCII values of 'h', 'e', 'l', 'l', 'o'
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_string_and_break() {
    let input = r#"
        let test_string = "hello"
        let sum = 0
        for i in test_string {
            if (i == "l") {
                break
            }
            sum = sum + 1
        }
        print(sum)  // Expect the sum of ASCII values of 'h', 'e'
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_string_and_continue() {
    let input = r#"
        let test_string = "hello"
        let sum = 0
        for i in test_string {
            if (i == "l") {
                continue
            }
            sum = sum + 1
        }
        print(sum)  // Expect the sum of ASCII values of 'h', 'e', 'o'
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_dict() {
    let input = r#"
        let test_dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
        let sum = 0

        for key, value in test_dict {
            print(key, value)
            sum = sum + value
        }
        print(sum)  // Expect 15
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_dict_and_break() {
    let input = r#"
        let test_dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
        let sum = 0

        for key, value in test_dict {
            if (key == "c") {
                break
            }
            sum = sum + value
        }
        print(sum)  // Expect 3
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_dict_and_continue() {
    let input = r#"
        let test_dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
        let sum = 0

        for key, value in test_dict {
            if (key == "c") {
                continue
            }
            sum = sum + value
        }
        print(sum)  // Expect 12
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop_with_dict_and_key_value() {
    let input = r#"
        let test_dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
        let sum = 0

        // Unsupported, where HashMap should have a key-value pair
        for key in test_dict {
            sum = sum + test_dict[key]
        }
        print(sum)  // Expect 15
    "#;
    let result = run_script(input);
    assert!(result.is_err());
}

#[test]
fn test_for_loop_with_dict_and_key_value_and_break() {
    let input = r#"
        let test_dict = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
        let sum = 0

        for key, value in test_dict {
            if (key == "c") {
                break
            }
            sum = sum + value
        }
        print(sum)  // Expect 3
    "#;
    let result = run_script(input);
    assert!(result.is_ok());
}
