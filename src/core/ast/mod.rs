mod syntax;

use self::syntax::is;

pub enum SyntaxType {
    Headline,
    Paragraph,
    Table,
}

pub fn inspect_syntax_type(syntax: SyntaxType) -> String {
    match syntax {
        SyntaxType::Headline => return "Headline".to_string(),
        SyntaxType::Paragraph => return "Paragraph".to_string(),
        SyntaxType::Table => return "Table".to_string(),
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AST {
    pub syntax: String,
    pub content: String,
}

impl AST {
    fn new(syntax_type: SyntaxType, content: String) -> Self {
        AST {
            syntax: inspect_syntax_type(syntax_type),
            content: content,
        }
    }
}

pub fn compile_md_to_ast(md: &str) -> Vec<AST> {
    let ast = vec![];
    let result = syntax::is::headline(md);
    return ast;
}
