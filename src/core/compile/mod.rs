use crate::core::ast::compile_md_to_ast;

pub fn compile_md_to_html(md: &str) -> String {
    let vec_ast = compile_md_to_ast(md);

    // println!("vec_ast = {:?}", vec_ast);
    return "hello ast".to_string();
}
