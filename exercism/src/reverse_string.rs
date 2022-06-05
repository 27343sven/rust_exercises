#[allow(unused)]



pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

#[test]
fn reverse_cool() {
    assert_eq!("looc".to_string(), reverse("cool"))
}
