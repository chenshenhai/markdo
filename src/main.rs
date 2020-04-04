use markdo;
use std::fs::File;
use std::path::Path;
use std::io::Read;


fn main() {
    let file_path = "tests/assets/md.txt";
    let mut md = String::new();
    File::open(Path::new(&file_path)).unwrap().read_to_string(&mut md).unwrap();
    
    let html = markdo::parse(&md);
    println!("html = {}", html);
}