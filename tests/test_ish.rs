use ish::ish;

#[test]
fn basic_boolean_test() {
    assert_eq!(true - ish, "true");
    assert!(true - ish != "false");
}

#[test]
fn basic_float_test() {
    assert_eq!(0.01 - ish, 0.01 - 0.000000000001);
    println!("{:?}", 0.01 - ish);
    assert!(0.01 - ish != 3.0);
}
