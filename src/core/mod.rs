mod ast;
mod compile;
use self::compile::compile_md_to_html;

pub fn compile(md: &str) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        println!("lines[i] = {:?}", lines[i]);
        i += 1;
    }
    return compile_md_to_html(md);
}
