use ish::ish;

#[test]
fn basic_sanity_test() {
    assert_eq!(true - ish, "true");
    assert!(true - ish != "false");
}
