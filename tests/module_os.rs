use nikl::run_script;

#[test]
fn test_os_get_cwd() {
    let input = r#"
        import "os" as os
        let cwd = os.get_cwd()
        print(cwd)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_set_get_cwd() {
    let input = r#"
        import "os" as os
        let original = os.get_cwd()
        os.set_cwd(".")
        let new = os.get_cwd()
        print(original)
        print(new)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_list_dir() {
    let input = r#"
        import "os" as os
        let items = os.list_dir(".")
        print(items)
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_make_and_remove_dir() {
    let input = r#"
        import "os" as os
        os.make_dir("test_dir")
        let exists = os.exists("test_dir")
        print(exists)
        os.remove_dir("test_dir")
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_remove_file() {
    let input = r#"
        import "os" as os
        os.write_file("temp.txt", "hello")
        os.remove_file("temp.txt")
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_rename_file() {
    let input = r#"
        import "os" as os
        os.write_file("old.txt", "data")
        os.rename("old.txt", "new.txt")
        os.remove_file("new.txt")
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_exists_is_file_is_dir() {
    let input = r#"
        import "os" as os
        os.write_file("file.txt", "hi")
        os.make_dir("dir_check")
        print(os.exists("file.txt"))
        print(os.is_file("file.txt"))
        print(os.is_dir("dir_check"))
        os.remove_file("file.txt")
        os.remove_dir("dir_check")
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_read_write_file() {
    let input = r#"
        import "os" as os
        os.write_file("file.txt", "hello world")
        let content = os.read_file("file.txt")
        print(content)
        os.remove_file("file.txt")
    "#;
    assert!(run_script(input).is_ok());
}

#[test]
fn test_os_env_get_set() {
    let input = r#"
        import "os" as os
        os.env_set("TEST_ENV", "nikl")
        let val = os.env_get("TEST_ENV")
        print(val)
    "#;
    assert!(run_script(input).is_ok());
}
