
pub enum SyntaxType {
    Headline,
    Paragraph,
    Table,
}

pub fn inspect_syntax_type(syntax: SyntaxType) -> String {
    match syntax {
        SyntaxType::Headline => return "Headline".to_string(),
        SyntaxType::Paragraph => return "Paragraph".to_string(),
        SyntaxType::Table => return "Table".to_string()
    }
}

pub struct AST {
    pub syntax: SyntaxType,
    pub content: String
}

impl AST {
    fn new(syntax: SyntaxType, content: String) -> Self  {
        AST {
            syntax: syntax,
            content: content,
        }
    }
}


pub fn compile_md_to_ast(md: &str) -> Vec<AST> {
    let ast = vec![];
    return ast;
}