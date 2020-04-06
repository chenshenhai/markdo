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

pub fn compile_ast_to_html(asts: &Vec<AST>) -> String {
    // TODO
    let mut str_list: Vec<String> = vec![];
    for i in 0..asts.len() as usize {
        println!("ast = {:?}", asts[i]);
        if asts[i].syntax.eq("H1") {
            let line = "<h1>".to_string() + &asts[i].content + &"</h1>".to_string();
            str_list.push(line.to_string());
        }
    }
    let html = str_list.join("");
    return html;
}
