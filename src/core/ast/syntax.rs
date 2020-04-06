pub mod judge {
    pub fn syntax(stri: &str) -> u8 {
        let mut code = 0;
        let set = regex::RegexSet::new(&[
            r"\w+",                          // 0: paragraph
            r"^\#{1,1}[\s]{1,}([^\#\n\s]+)", // 2: h1
            r"^\#{2,2}[\s]{1,}([^\#\n\s]+)", // 2: h2
            r"^\#{3,3}[\s]{1,}([^\#\n\s]+)", // 3: h3
            r"^\#{4,4}[\s]{1,}([^\#\n\s]+)", // 4: h4
            r"^\#{5,5}[\s]{1,}([^\#\n\s]+)", // 5: h5
            r"^\#{6,6}[\s]{1,}([^\#\n\s]+)", // 6: h6
        ])
        .unwrap();
        let matches: Vec<_> = set.matches(stri).into_iter().collect();
        if matches.len() > 1 {
            code = matches[1] as u8;
        }
        return code;
    }
}

pub mod is {

    pub fn paragraph(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 6 {
            result = true;
        }
        return result;
    }

    pub fn headline(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code >= 1 && code <= 6 {
            result = true;
        }
        return result;
    }

    pub fn h1(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 1 {
            result = true;
        }
        return result;
    }

    pub fn h2(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 2 {
            result = true;
        }
        return result;
    }

    pub fn h3(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 3 {
            result = true;
        }
        return result;
    }

    pub fn h4(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 4 {
            result = true;
        }
        return result;
    }

    pub fn h5(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 5 {
            result = true;
        }
        return result;
    }

    pub fn h6(line: &str) -> bool {
        let code = super::judge::syntax(line);
        let mut result = false;
        if code == 6 {
            result = true;
        }
        return result;
    }
}

pub mod capture {
    pub fn headline(stri: &str) -> String {
        let re = regex::Regex::new(r"^\#{1,6}[\s]{1,}([^\#\n\s]+)").unwrap();
        let result = re.captures(stri);
        println!("capture.result = {:?}", result);
        return "".to_string();
    }
}
