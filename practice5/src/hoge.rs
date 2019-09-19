pub fn foo() -> String {
    "FOO!!!!!".to_string()
}

pub fn foobar() -> String {
    format!("{}{}", foo(), bar())
}

fn bar() -> String {
    "BAR!!!!!".to_string()
}

// unit test hoge::tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_unittests() {
        assert_eq!(foo(), "FOO!!!!!".to_string());
    }

    // private method can be tested
    #[test]
    fn bar_unittests() {
        assert_eq!(bar(), "BAR!!!!!".to_string());
    }
}
