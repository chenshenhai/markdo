use markdo;

#[test]
fn it_parse() {
    let md = "hello world".to_string();
    assert_eq!("hello world", markdo::parse(&md));
}