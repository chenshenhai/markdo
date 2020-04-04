mod core;
use crate::core::compile;

pub fn parse(md: &str) -> String{
    let result = compile(md);
    println!("result,md = {}", result);
    let result = "hello world".to_string();
    return result;
}
