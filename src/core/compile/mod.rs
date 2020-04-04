mod ast;
use crate::core::compile::ast::compile_md_to_ast;

pub fn compile_md_to_html(md: &str) -> String {
    let vec_ast = compile_md_to_ast(md);
    return "".to_string();
}

