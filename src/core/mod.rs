mod compile;
use crate::core::compile::compile_md_to_html;

pub fn compile(md: &str) -> String {
    return compile_md_to_html(md);
}


