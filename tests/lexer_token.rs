use nikl::lexer::Lexer;


#[test]
fn test_lexer() {
    let input = r#"
        let x = 10
        const y = 20.5
        print("Hello, World!")
        
        if x < y {
            print("x is less than y")
        } else {
            print("x is greater than or equal to y")
        }
        
        if x == 10 {
            print("x is equal to 10")
        } else if x > 10 {
            print("x is greater than 10")
        } else {
            print("x is less than 10")
        }
        (not True, False, spawn, wait)
        if (x > 0) {
            print("x is positive")
        }

        fn add(a, b) {
            a = a + 1
            b = b + 1
            print(a) print(b)
            return a + b
        }

        fn tests() {
            const and_test = True and False
            const or_test = True or False
            const not_test = not True

            print(and_test)
            print(or_test)
            print(not_test)
        }
    "#;

    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    for token in tokens {
        println!("{:?}", token);
    }
}
