// TRPL uses &String for the argument
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        // byte is dereferenced, not b' ' is referenced
        // If statement is working like pattern match,
        // so instead of 'append'ing & at b' ',
        // the content of the reference is 'pick'ed up
        if byte == b' ' {
            return &s[0..i];
        }
    }

    s
}

#[test]
fn first_word_test() {
    assert_eq!(first_word("Hello world!"), "Hello");
    assert_eq!(first_word("aww"), "aww");
}
