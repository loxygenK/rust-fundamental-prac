fn word(s: &str, index: usize) -> Option<&str> {
    let delimiters: Vec<usize> = s.bytes()
        .enumerate()
        .filter(|(_, b)| b == &b' ')
        .map(|(i, _)| i)
        .chain([s.len()].iter().copied()) // I believe there must be a better way
        .collect();

    if index > delimiters.len() - 1 {
        return None;
    }

    if index == 0 {
        return Some(&s[..delimiters[0]])
    }

   Some(&s[(delimiters[index - 1] + 1)..delimiters[index]])
}

#[test]
fn word_test() {
    assert_eq!(word("Hello, world!", 0), Some("Hello,"));
    assert_eq!(word("Hello, world!", 1), Some("world!"));

    assert_eq!(word("Aww", 0), Some("Aww"));

    assert_eq!(word("beyond", 1), None)
}
