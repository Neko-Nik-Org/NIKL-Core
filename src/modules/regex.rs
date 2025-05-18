use regex::Regex;
use crate::interpreter::value::Value;


pub fn make_module() -> Value {
    let items = vec![
        (Value::String("match".to_string()), Value::BuiltinFunction(re_match)),
        (Value::String("is_match".to_string()), Value::BuiltinFunction(re_is_match)),
        (Value::String("find_all".to_string()), Value::BuiltinFunction(re_findall)),
        (Value::String("replace".to_string()), Value::BuiltinFunction(re_replace)),
    ];
    Value::HashMap(items)
}

fn re_is_match(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("is_match expects 2 arguments: pattern, text".to_string());
    }

    if let (Value::String(pat), Value::String(text)) = (&args[0], &args[1]) {
        Regex::new(pat)
            .map_err(|e| format!("regex error: {}", e))
            .map(|re| Value::Bool(re.is_match(text)))
    } else {
        Err("is_match expects two string arguments".to_string())
    }
}

fn re_match(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("match expects 2 arguments: pattern, text".to_string());
    }

    if let (Value::String(pat), Value::String(text)) = (&args[0], &args[1]) {
        Regex::new(pat)
            .map_err(|e| format!("regex error: {}", e))
            .and_then(|re| {
                if let Some(caps) = re.captures(text) {
                    let matches = caps
                        .iter()
                        .map(|m| match m {
                            Some(m) => Value::String(m.as_str().to_string()),
                            None => Value::Null,
                        })
                        .collect();
                    Ok(Value::Array(matches))
                } else {
                    Ok(Value::Null)
                }
            })
    } else {
        Err("match expects two string arguments".to_string())
    }
}

fn re_findall(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("findall expects 2 arguments: pattern, text".to_string());
    }

    if let (Value::String(pat), Value::String(text)) = (&args[0], &args[1]) {
        Regex::new(pat)
            .map_err(|e| format!("regex error: {}", e))
            .map(|re| {
                let matches = re
                    .find_iter(text)
                    .map(|m| Value::String(m.as_str().to_string()))
                    .collect();
                Value::Array(matches)
            })
    } else {
        Err("findall expects two string arguments".to_string())
    }
}

fn re_replace(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("replace expects 3 arguments: pattern, replacement, text".to_string());
    }

    if let (Value::String(pat), Value::String(repl), Value::String(text)) = (&args[0], &args[1], &args[2]) {
        Regex::new(pat)
            .map_err(|e| format!("regex error: {}", e))
            .map(|re| Value::String(re.replace_all(text, repl.as_str()).to_string()))
    } else {
        Err("replace expects three string arguments".to_string())
    }
}
