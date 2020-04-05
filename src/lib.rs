mod core;
extern crate regex;
use crate::core::compile;

pub fn parse(md: &str) -> String {
    let result = compile(md);
    return result;
}
