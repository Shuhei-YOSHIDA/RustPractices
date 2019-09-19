// integration test
// Use this crate just external crate, hence private method cannot be used

extern crate practice5;

#[test]
fn integration_test01() {
    assert_eq!(practice5::hoge::foo(), "FOO!!!!!".to_string());
}
