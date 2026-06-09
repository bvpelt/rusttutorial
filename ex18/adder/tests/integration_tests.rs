use adder::add_five;

mod common;

#[test]
fn add_five_test() {
    common::setup();

    let result = add_five(2);
    assert_eq!(result, 7);
}
