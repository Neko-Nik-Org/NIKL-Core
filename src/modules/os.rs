use std::{fs, env};

use crate::interpreter::value::Value;


pub fn make_os_module() -> Value {
    let items = vec![
        (Value::String("get_cwd".to_string()), Value::BuiltinFunction(os_getcwd)),
        (Value::String("list_dir".to_string()), Value::BuiltinFunction(os_listdir)),
    ];
    Value::HashMap(items)
}

fn os_getcwd(_: Vec<Value>) -> Result<Value, String> {
    env::current_dir()
        .map(|p| Value::String(p.to_string_lossy().to_string()))
        .map_err(|e| format!("os.getcwd error: {}", e))
}

fn os_listdir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("listdir expects 1 argument".to_string());
    }

    if let Value::String(path) = &args[0] {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("os.listdir error: {}", e))?;

        let files = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| Value::String(entry.file_name().to_string_lossy().to_string()))
            .collect();

        Ok(Value::Array(files))
    } else {
        Err("listdir expects a string path".to_string())
    }
}
