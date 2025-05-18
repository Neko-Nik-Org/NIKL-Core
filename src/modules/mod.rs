pub mod builtin_core;
mod os;
mod regex;

pub use os::make_module as make_os_module;
pub use regex::make_module as make_regex_module;
