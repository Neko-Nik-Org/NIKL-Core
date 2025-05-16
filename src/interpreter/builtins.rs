//! Built-in functions for the interpreter
//! These functions are available in the interpreter environment
//! and can be called directly from the user code

use super::engine::Value;


/// Built-in function to print values to the console
/// It accepts any number of arguments and prints them in a single line
pub fn builtin_print(args: Vec<Value>) -> Result<Value, String> {
    let output: Vec<String> = args
        .into_iter()
        .map(|v| match v {
            Value::Bool(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s,
            Value::Null => "None".to_string(),
            _ => "<unprintable>".to_string(),
        })
        .collect();

    println!("{}", output.join(" "));
    Ok(Value::Null)
}


/// Built-in function to get the length of any possible type
/// Currently only works on strings
pub fn builtin_len(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("len() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
        _ => Err("len() currently only works on strings".to_string()),
    }
}
