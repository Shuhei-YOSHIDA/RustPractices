pub mod hoge;
pub mod neko;

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    // can write this in hoge.rs
    #[test]
    fn foo_tests() {
        assert_eq!(hoge::foo(), "FOO!!!!!".to_string());
    }
}
