use crate::core::ast::compile_md_to_ast;
// use regex::Regex;

pub fn compile_md_to_html(md: &str) -> String {
    let vec_ast = compile_md_to_ast(md);
    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    println!("vec_ast = {:?}", vec_ast);
    return "hello ast".to_string();
}
