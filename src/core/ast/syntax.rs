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

pub enum SyntaxType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    // Table,
}

pub fn inspect_syntax_type(syntax: SyntaxType) -> String {
    match syntax {
        SyntaxType::H1 => return "H1".to_string(),
        SyntaxType::H2 => return "H2".to_string(),
        SyntaxType::H3 => return "H3".to_string(),
        SyntaxType::H4 => return "H4".to_string(),
        SyntaxType::H5 => return "H5".to_string(),
        SyntaxType::H6 => return "H6".to_string(),
        SyntaxType::P => return "P".to_string(),
        // SyntaxType::Table => return "Table".to_string(),
    }
}

pub mod judge {
    pub fn syntax(stri: &str) -> u8 {
        let mut code = 0;
        let set = regex::RegexSet::new(&[
            r"\w+",                        // 0: paragraph
            r"^\#{1,1}[\s]{1,}([^\#\n]+)", // 2: h1
            r"^\#{2,2}[\s]{1,}([^\#\n]+)", // 2: h2
            r"^\#{3,3}[\s]{1,}([^\#\n]+)", // 3: h3
            r"^\#{4,4}[\s]{1,}([^\#\n]+)", // 4: h4
            r"^\#{5,5}[\s]{1,}([^\#\n]+)", // 5: h5
            r"^\#{6,6}[\s]{1,}([^\#\n]+)", // 6: h6
        ])
        .unwrap();
        let matches: Vec<_> = set.matches(stri).into_iter().collect();
        if matches.len() > 1 {
            code = matches[1] as u8;
        }
        return code;
    }
}

pub fn create_ast(line: &str) -> AST {
    let code = self::judge::syntax(line);
    let mut syntax_type: SyntaxType = SyntaxType::P;
    let mut content = line.to_string();
    match code {
        1 => {
            syntax_type = SyntaxType::H1;
            content = self::capture::headline(line);
        }
        2 => {
            syntax_type = SyntaxType::H2;
            content = self::capture::headline(line);
        }
        3 => {
            syntax_type = SyntaxType::H3;
            content = self::capture::headline(line);
        }
        4 => {
            syntax_type = SyntaxType::H4;
            content = self::capture::headline(line);
        }
        5 => {
            syntax_type = SyntaxType::H5;
            content = self::capture::headline(line);
        }
        6 => {
            syntax_type = SyntaxType::H6;
            content = self::capture::headline(line);
        }
        _ => {}
    }
    let ast = AST::new(syntax_type, content);
    return ast;
}

pub mod capture {

    pub fn headline(stri: &str) -> String {
        let re = regex::Regex::new(r"^\#{1,6}[\s]{1,}([^\#\n]+)").unwrap();
        let result = re.captures_iter(stri);
        let mut content = "".to_string();
        for cap in result {
            content = cap[1].to_string();
        }
        return content;
    }
}
