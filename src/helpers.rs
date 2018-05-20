pub fn lowercase_no_whitespace(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut s = String::new();

    for &byte in bytes.iter() {
        let c = byte as char;

        if !c.is_whitespace() {
            s.push(c);
        }
    }

    s.to_lowercase()
}
