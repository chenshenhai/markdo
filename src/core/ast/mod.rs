mod syntax;
use self::syntax::AST;

pub fn compile_md_to_ast(md: &str) -> Vec<AST> {
    let lines: Vec<&str> = md.lines().collect();
    let mut i = 0;
    let mut asts: Vec<AST> = vec![];
    while i < lines.len() {
        let line = lines[i];
        let ast = syntax::create_ast(line);
        asts.push(ast);
        i += 1;
    }
    return asts;
}


pub fn compile_ast_to_html() {
    // TODO
}