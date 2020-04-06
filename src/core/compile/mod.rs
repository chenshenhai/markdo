use crate::core::ast::compile_md_to_ast;

pub fn compile_md_to_html(md: &str) -> String {
    let asts = compile_md_to_ast(md);
    println!("asts = {:?}", asts);
    return "hello ast".to_string();
}
