use nikl::run_script;

#[test]
fn test_regex_is_match() {
    let input = r#"
        import "regex" as regex
        let matched = regex.is_match("\d+", "123abc")
        print(matched)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_regex_match_groups() {
    let input = r#"
        import "regex" as regex
        let m = regex.match("(\d+)-(\w+)", "123-abc")
        print(m)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_regex_findall() {
    let input = r#"
        import "regex" as regex
        let all = regex.find_all("\w+", "a b c123")
        print(all)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_regex_replace() {
    let input = r#"
        import "regex" as regex
        let result = regex.replace("\d+", "NUM", "abc123def456")
        print(result)
    "#;
    assert!(run_script(input).is_ok());
}
