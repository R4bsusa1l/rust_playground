use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // Call the setup function from common module
    let result = add_two(2);
    assert_eq!(result, 4);
}

// $ cargo test --test integration_test     -> To run all the tests in a particular integration test file