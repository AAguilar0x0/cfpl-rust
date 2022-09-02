mod common;

#[test]
fn no_error() {
    common::no_input(true, "./test_source_codes/no_input/no_error/");
}

#[test]
fn with_error() {
    common::no_input(false, "./test_source_codes/no_input/with_error/");
}
