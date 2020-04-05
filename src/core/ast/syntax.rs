pub mod is {
    pub fn headline(line: &str) -> bool {
        let re = regex::Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        const TO_SEARCH: &'static str = "
        On 2010-03-14, foo happened. On 2014-10-14, bar happened.
        ";
        for caps in re.captures_iter(TO_SEARCH) {
            // Note that all of the unwraps are actually OK for this regex
            // because the only way for the regex to match is if all of the
            // capture groups match. This is not true in general though!
            println!("year: {}, month: {}, day: {}",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                    caps.get(3).unwrap().as_str());
        }

        // let mat = re.find("phone: 111-222-3333").unwrap();
        // assert_eq!((mat.start(), mat.end()), (7, 19));
        let result = true;
        return result;
    }
}
