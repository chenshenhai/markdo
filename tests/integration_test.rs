use markdo;
use std::fs::File;
use std::path::Path;
use std::io::Read;


#[test]
fn it_parse() {

    let file_path = "tests/assets/md.txt";
    let mut buffer = String::new();
    File::open(Path::new(&file_path)).unwrap().read_to_string(&mut buffer).unwrap();
    println!("buffer = {}", buffer);

    let md = "hello world".to_string();
    assert_eq!("hello world", markdo::parse(&md));
}