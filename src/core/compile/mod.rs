use crate::core::ast;

pub fn compile_md_to_html(md: &str) -> String {
    let asts = ast::compile_md_to_ast(md);
    let html = ast::compile_ast_to_html(&asts);
    println!("html = {:?}", html);
    return html;
}
