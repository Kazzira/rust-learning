pub trait ToPlural {
    fn to_plural(&self) -> String;
}

impl ToPlural for String {
    fn to_plural(&self) -> String {
        let mut plural = self.clone();
        if plural.ends_with('s') {
            plural.push_str("es");
        } else {
            plural.push('s');
        }
        plural
    }
}

#[test]
fn test_file_to_plural() {
    let word = "file".to_string();
    let plural_word = word.to_plural();
    assert_eq!(plural_word, "files");
}
