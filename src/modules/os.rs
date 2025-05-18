use std::{env, fs, path::Path};

use crate::interpreter::value::Value;


pub fn make_module() -> Value {
    let items = vec![
        (Value::String("get_cwd".to_string()), Value::BuiltinFunction(get_cwd)),
        (Value::String("set_cwd".to_string()), Value::BuiltinFunction(set_cwd)),
        (Value::String("list_dir".to_string()), Value::BuiltinFunction(list_dir)),
        (Value::String("make_dir".to_string()), Value::BuiltinFunction(make_dir)),
        (Value::String("remove_dir".to_string()), Value::BuiltinFunction(remove_dir)),
        (Value::String("remove_file".to_string()), Value::BuiltinFunction(remove_file)),
        (Value::String("rename".to_string()), Value::BuiltinFunction(rename)),
        (Value::String("exists".to_string()), Value::BuiltinFunction(exists)),
        (Value::String("is_file".to_string()), Value::BuiltinFunction(is_file)),
        (Value::String("is_dir".to_string()), Value::BuiltinFunction(is_dir)),
        (Value::String("read_file".to_string()), Value::BuiltinFunction(read_file)),
        (Value::String("write_file".to_string()), Value::BuiltinFunction(write_file)),
        (Value::String("env_get".to_string()), Value::BuiltinFunction(env_get)),
        (Value::String("env_set".to_string()), Value::BuiltinFunction(env_set)),
    ];
    Value::HashMap(items)
}


fn get_cwd(_: Vec<Value>) -> Result<Value, String> {
    env::current_dir()
        .map(|p| Value::String(p.to_string_lossy().to_string()))
        .map_err(|e| format!("os.getcwd error: {}", e))
}

fn set_cwd(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        env::set_current_dir(path)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.set_cwd error: {}", e))
    } else {
        Err("setcwd expects a string path".to_string())
    }
}

fn list_dir(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
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

fn make_dir(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        fs::create_dir_all(path)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.mkdir error: {}", e))
    } else {
        Err("mkdir expects a string path".to_string())
    }
}

fn remove_dir(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        fs::remove_dir_all(path)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.rmdir error: {}", e))
    } else {
        Err("rmdir expects a string path".to_string())
    }
}

fn remove_file(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        fs::remove_file(path)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.remove_file error: {}", e))
    } else {
        Err("remove_file expects a string path".to_string())
    }
}

fn rename(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("rename expects 2 arguments: old_path, new_path".to_string());
    }
    if let (Value::String(src), Value::String(dst)) = (&args[0], &args[1]) {
        fs::rename(src, dst)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.rename error: {}", e))
    } else {
        Err("rename expects 2 string arguments".to_string())
    }
}

fn exists(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        Ok(Value::Bool(Path::new(path).exists()))
    } else {
        Err("exists expects a string path".to_string())
    }
}

fn is_file(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        Ok(Value::Bool(Path::new(path).is_file()))
    } else {
        Err("is_file expects a string path".to_string())
    }
}

fn is_dir(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        Ok(Value::Bool(Path::new(path).is_dir()))
    } else {
        Err("is_dir expects a string path".to_string())
    }
}

fn read_file(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(path)) = args.get(0) {
        fs::read_to_string(path)
            .map(Value::String)
            .map_err(|e| format!("os.read_file error: {}", e))
    } else {
        Err("read_file expects a string path".to_string())
    }
}

fn write_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("write_file expects 2 arguments: path, content".to_string());
    }
    if let (Value::String(path), Value::String(content)) = (&args[0], &args[1]) {
        fs::write(path, content)
            .map(|_| Value::Null)
            .map_err(|e| format!("os.write_file error: {}", e))
    } else {
        Err("write_file expects 2 string arguments".to_string())
    }
}

fn env_get(args: Vec<Value>) -> Result<Value, String> {
    if let Some(Value::String(key)) = args.get(0) {
        Ok(env::var(key).map_or(Value::Null, Value::String))
    } else {
        Err("env_get expects a string key".to_string())
    }
}

fn env_set(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("env_set expects 2 arguments: key, value".to_string());
    }
    if let (Value::String(key), Value::String(val)) = (&args[0], &args[1]) {
        unsafe {
            env::set_var(key, val);
        }
        Ok(Value::Null)
    } else {
        Err("env_set expects 2 string arguments".to_string())
    }
}
